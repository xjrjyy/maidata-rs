use super::Note;
use crate::insn;
use crate::materialize::{
    MaterializedHold, MaterializedSlideSegment, MaterializedSlideSegmentGroup,
    MaterializedSlideTrack, MaterializedTap, MaterializedTapShape, MaterializedTouch,
    MaterializedTouchHold,
};
use crate::transform;

pub struct MaterializationContext {
    // TODO: is slides' default stop time really independent of BPM changes?
    // currently it is dependent -- add a separate non-changing value (initialized by the "wholebpm"
    // thing) to move to independent
    curr_beat_dur: f32,
    curr_note_dur: f32,
    curr_ts: f32,
}

impl MaterializationContext {
    pub fn with_offset(offset_secs: f32) -> Self {
        Self {
            curr_beat_dur: 0.0,
            curr_note_dur: 0.0,
            curr_ts: offset_secs,
        }
    }

    /// Materialize a list of raw instructions into notes.
    pub fn materialize_insns<'a, I: IntoIterator<Item = &'a crate::Sp<insn::RawInsn>>>(
        &mut self,
        insns: I,
    ) -> Vec<Note> {
        insns
            .into_iter()
            .flat_map(|insn| self.materialize_raw_insn(insn))
            .collect()
    }

    /// Read in one raw instruction and materialize into note(s) if applicable.
    fn materialize_raw_insn(&mut self, insn: &crate::Sp<insn::RawInsn>) -> Vec<Note> {
        use std::ops::Deref;
        match insn.deref() {
            insn::RawInsn::Bpm(params) => {
                self.set_bpm(params.new_bpm);
                vec![]
            }
            insn::RawInsn::BeatDivisor(params) => {
                match params {
                    insn::BeatDivisorParams::NewDivisor(new_divisor) => {
                        self.set_beat_divisor(*new_divisor);
                    }
                    insn::BeatDivisorParams::NewAbsoluteDuration(new_note_dur) => {
                        self.curr_note_dur = *new_note_dur;
                    }
                }
                vec![]
            }
            insn::RawInsn::Rest => {
                // currently rests don't materialize to anything
                let _ = self.advance_time();
                vec![]
            }
            insn::RawInsn::EndMark => {
                // TODO: make later materialize calls return error?
                vec![]
            }
            insn::RawInsn::Note(raw_note) => {
                let ts = self.advance_time();
                self.materialize_raw_note(ts, raw_note)
            }
            insn::RawInsn::NoteBundle(raw_notes) => {
                let ts = self.advance_time();
                raw_notes
                    .iter()
                    .flat_map(|raw_note| self.materialize_raw_note(ts, raw_note))
                    .collect()
            }
        }
    }

    fn set_bpm(&mut self, new_bpm: f32) {
        self.curr_beat_dur = bpm_to_beat_dur(new_bpm);
    }

    fn set_beat_divisor(&mut self, new_divisor: u32) {
        self.curr_note_dur = divide_beat(self.curr_beat_dur, new_divisor);
    }

    /// Advances timestamp by one "note", return the timestamp before advancing (that of the
    /// current note being materialized).
    fn advance_time(&mut self) -> f32 {
        let res = self.curr_ts;
        self.curr_ts += self.curr_note_dur;
        res
    }

    fn materialize_raw_note(&self, ts: f32, raw_note: &insn::RawNoteInsn) -> Vec<Note> {
        match raw_note {
            insn::RawNoteInsn::Tap(params) => {
                let m_params = materialize_tap_params(ts, params, false);
                vec![Note::Tap(m_params)]
            }
            insn::RawNoteInsn::Touch(params) => {
                let m_params = materialize_touch_params(ts, params);
                vec![Note::Touch(m_params)]
            }
            insn::RawNoteInsn::Slide(params) => materialize_slide(ts, self.curr_beat_dur, params),
            insn::RawNoteInsn::Hold(params) => {
                let m_params = materialize_hold_params(ts, self.curr_beat_dur, params);
                vec![Note::Hold(m_params)]
            }
            insn::RawNoteInsn::TouchHold(params) => {
                let m_params = materialize_touch_hold_params(ts, self.curr_beat_dur, params);
                vec![Note::TouchHold(m_params)]
            }
        }
    }
}

fn bpm_to_beat_dur(bpm: f32) -> f32 {
    60.0 / bpm
}

fn divide_beat(beat_dur: f32, beat_divisor: u32) -> f32 {
    beat_dur * 4.0 / (beat_divisor as f32)
}

fn materialize_tap_params(ts: f32, p: &insn::TapParams, is_slide_star: bool) -> MaterializedTap {
    let shape = match is_slide_star {
        false => MaterializedTapShape::Ring,
        true => MaterializedTapShape::Star,
    };

    MaterializedTap {
        ts,
        key: p.key,
        shape,
        is_break: p.is_break,
        is_ex: p.is_ex,
    }
}

fn materialize_touch_params(ts: f32, p: &insn::TouchParams) -> MaterializedTouch {
    MaterializedTouch {
        ts,
        sensor: p.sensor,
    }
}

/// slide insn -> `vec![star tap, track, track, ...]`
fn materialize_slide(ts: f32, beat_dur: f32, p: &insn::SlideParams) -> Vec<Note> {
    // star
    let star = Note::Tap(materialize_tap_params(ts, &p.start, true));
    let start_key = p.start.key;

    let tracks = p
        .tracks
        .iter()
        .map(|track| Note::SlideTrack(materialize_slide_track(ts, beat_dur, start_key, track)));

    let mut result = Vec::with_capacity(tracks.len() + 1);
    result.push(star);
    result.extend(tracks);
    result
}

fn materialize_slide_track(
    ts: f32,
    beat_dur: f32,
    start_key: insn::Key,
    track: &insn::SlideTrack,
) -> MaterializedSlideTrack {
    // in simai, stop time is actually encoded (overridden) in the duration spec of individual
    // slide track
    //
    // take care of this, falling back to beat duration of current bpm
    let stop_time = match track.groups.last().unwrap().len {
        insn::SlideLength::Simple(_) => beat_dur,
        insn::SlideLength::Custom(st, _) => stop_time_spec_to_dur(st),
    };

    let start_ts = ts + stop_time;

    let mut start_key = start_key;
    let groups = track
        .groups
        .iter()
        .map(|group| {
            let result = materialize_slide_segment_group(beat_dur, start_key, group);
            // TODO: trait for slide end position
            start_key = group.segments.last().unwrap().params().destination;
            result
        })
        .collect();

    MaterializedSlideTrack {
        ts,
        start_ts,
        groups,
        is_break: track.is_break,
    }
}

fn materialize_slide_segment_group(
    beat_dur: f32,
    start: insn::Key,
    group: &insn::SlideSegmentGroup,
) -> MaterializedSlideSegmentGroup {
    let mut start = start;
    let segments = group
        .segments
        .iter()
        .map(|segment| {
            let result = materialize_slide_segment(start, segment);
            // TODO: trait for slide end position
            start = segment.params().destination;
            result
        })
        .collect();

    MaterializedSlideSegmentGroup {
        dur: materialize_duration(group.len.slide_duration(), beat_dur),
        segments,
    }
}

fn materialize_slide_segment(
    start: insn::Key,
    segment: &insn::SlideSegment,
) -> MaterializedSlideSegment {
    // TODO: handle normalization error
    let segment = transform::normalize::normalize_slide_segment(start, segment).unwrap();
    let shape = segment.shape();
    let params = segment.params();

    MaterializedSlideSegment {
        start: params.start,
        destination: params.destination,
        shape,
    }
}

fn materialize_hold_params(ts: f32, beat_dur: f32, p: &insn::HoldParams) -> MaterializedHold {
    MaterializedHold {
        ts,
        dur: materialize_duration(p.len, beat_dur),
        key: p.key,
        is_break: p.is_break,
        is_ex: p.is_ex,
    }
}

fn materialize_touch_hold_params(
    ts: f32,
    beat_dur: f32,
    p: &insn::TouchHoldParams,
) -> MaterializedTouchHold {
    MaterializedTouchHold {
        ts,
        dur: materialize_duration(p.len, beat_dur),
        sensor: p.sensor,
    }
}

fn materialize_duration(x: insn::Length, beat_dur: f32) -> f32 {
    match x {
        insn::Length::NumBeats(p) => divide_beat(beat_dur, p.divisor) * (p.num as f32),
        insn::Length::Seconds(x) => x,
    }
}

fn stop_time_spec_to_dur(x: insn::SlideStopTimeSpec) -> f32 {
    match x {
        insn::SlideStopTimeSpec::Bpm(override_bpm) => bpm_to_beat_dur(override_bpm),
        insn::SlideStopTimeSpec::Seconds(x) => x,
    }
}
