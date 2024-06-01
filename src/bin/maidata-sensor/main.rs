use maidata::container::{AssociatedBeatmapData, Maidata};
use std::time::Instant;
use walkdir::WalkDir;

fn main() {
    let dir = std::env::args().nth(1).expect("usage: $0 <path/to/charts>");

    let start = Instant::now();

    let maidata_vec = WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir() && e.file_name() == "maidata.txt")
        .map(|e| read_file(e.path()))
        .map(|content| maidata::container::lex_maidata(&content))
        .collect::<Vec<_>>();
    let beatmap_data_vec = maidata_vec
        .iter()
        .flat_map(parse_maidata)
        .collect::<Vec<_>>();

    let mut result = beatmap_data_vec
        .iter()
        .map(|data| {
            let max_slide_length = data
                .groups
                .iter()
                .map(|group| {
                    group
                        .1
                        .iter()
                        .map(|note| -> DurationInSeconds {
                            let slide = match &note.raw_note {
                                MaterializedNote::SlideTrack(params) => params,
                                _ => return 0.0,
                            };
                            slide.groups.iter().map(|group| group.dur).sum()
                        })
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap_or(0.0)
                })
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0);
            (data, max_slide_length)
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (data, max_slide_length) in result.iter().take(20) {
        println!(
            "{:.2}s: {} [{:?}]",
            max_slide_length,
            data.maidata.title(),
            data.diff.difficulty()
        );
    }

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}

use maidata::insn::{Key, Position, TouchSensor};
use maidata::judge::slide_path::SLIDE_PATH_GETTER;
use maidata::materialize::{DurationInSeconds, Note as MaterializedNote, TimestampInSeconds};
use maidata::transform::transform::{Transformable, Transformer};
use maidata::transform::{
    NormalizedSlideSegment, NormalizedSlideSegmentGroup, NormalizedSlideSegmentParams,
    NormalizedSlideSegmentShape, NormalizedSlideTrack,
};
use maidata::Level;

const FRAMES_PER_SECOND: DurationInSeconds = 60.0;
const FRAME_DURATION: DurationInSeconds = 1.0 / FRAMES_PER_SECOND;

const TAP_JUDGE_THRESHOLD: DurationInSeconds = FRAME_DURATION * 9.0;
const SLIDE_JUDGE_THRESHOLD: DurationInSeconds = FRAME_DURATION * 3.0;

const GROUP_DUR_THRESHOLD: DurationInSeconds = 0.0001;

#[derive(Clone, Debug)]
struct Note {
    sensors: Vec<TouchSensor>,
    dur: std::ops::Range<TimestampInSeconds>,
    raw_note: MaterializedNote,
}

struct BeatmapData<'a> {
    maidata: &'a Maidata,
    diff: AssociatedBeatmapData<'a>,
    groups: Vec<(TimestampInSeconds, Vec<Note>)>,
}

fn key_to_sensor(key: Key) -> TouchSensor {
    ('A', key.index()).try_into().unwrap()
}

#[rustfmt::skip]
fn materialized_to_normalized_slide_segment(
    segment: &maidata::materialize::MaterializedSlideSegment,
) -> Vec<NormalizedSlideSegment> {
    if segment.shape == NormalizedSlideSegmentShape::Fan {
        return vec![
            NormalizedSlideSegment::Fan(NormalizedSlideSegmentParams {
                start: segment.start,
                destination: segment.destination.transform(Transformer {
                    rotation: 7,
                    flip: false,
                })
            }),
            NormalizedSlideSegment::Fan(NormalizedSlideSegmentParams {
                start: segment.start,
                destination: segment.destination,
            }),
            NormalizedSlideSegment::Fan(NormalizedSlideSegmentParams {
                start: segment.start,
                destination: segment.destination.transform(Transformer {
                    rotation: 1,
                    flip: false,
                })
            }),
        ]
    }
    let normalized_params = NormalizedSlideSegmentParams {
        start: segment.start,
        destination: segment.destination,
    };
    vec![match segment.shape {
        NormalizedSlideSegmentShape::Straight => NormalizedSlideSegment::Straight(normalized_params),
        NormalizedSlideSegmentShape::CircleL => NormalizedSlideSegment::CircleL(normalized_params),
        NormalizedSlideSegmentShape::CircleR => NormalizedSlideSegment::CircleR(normalized_params),
        NormalizedSlideSegmentShape::CurveL => NormalizedSlideSegment::CurveL(normalized_params),
        NormalizedSlideSegmentShape::CurveR => NormalizedSlideSegment::CurveR(normalized_params),
        NormalizedSlideSegmentShape::ThunderL => NormalizedSlideSegment::ThunderL(normalized_params),
        NormalizedSlideSegmentShape::ThunderR => NormalizedSlideSegment::ThunderR(normalized_params),
        NormalizedSlideSegmentShape::Corner => NormalizedSlideSegment::Corner(normalized_params),
        NormalizedSlideSegmentShape::BendL => NormalizedSlideSegment::BendL(normalized_params),
        NormalizedSlideSegmentShape::BendR => NormalizedSlideSegment::BendR(normalized_params),
        NormalizedSlideSegmentShape::SkipL => NormalizedSlideSegment::SkipL(normalized_params),
        NormalizedSlideSegmentShape::SkipR => NormalizedSlideSegment::SkipR(normalized_params),
        NormalizedSlideSegmentShape::Fan => unreachable!(),
    }]
}

fn parse_maidata(maidata: &Maidata) -> Vec<BeatmapData<'_>> {
    maidata
        .iter_difficulties()
        .map(move |diff| {
            // if !match diff.level() {
            //     Some(Level::Normal(level)) => (11..=13).contains(&level),
            //     Some(Level::Plus(level)) => (11..=13).contains(&level),
            //     Some(Level::Char(_)) => false,
            //     None => false,
            // } {
            //     continue;
            // }

            let mut mcx = maidata::materialize::MaterializationContext::with_offset(0.0);
            let notes = mcx.materialize_insns(diff.iter_insns());

            let mut notes = notes
                .into_iter()
                .map(|note| match &note {
                    MaterializedNote::Tap(params) => Note {
                        sensors: vec![key_to_sensor(params.key)],
                        dur: params.ts - TAP_JUDGE_THRESHOLD..params.ts,
                        raw_note: note,
                    },
                    MaterializedNote::Touch(params) => Note {
                        sensors: vec![params.sensor],
                        dur: params.ts - TAP_JUDGE_THRESHOLD..params.ts,
                        raw_note: note,
                    },
                    MaterializedNote::Hold(params) => Note {
                        sensors: vec![key_to_sensor(params.key)],
                        dur: params.ts - TAP_JUDGE_THRESHOLD..params.ts + params.dur, // TODO: check
                        raw_note: note,
                    },
                    MaterializedNote::TouchHold(params) => Note {
                        sensors: vec![params.sensor],
                        dur: params.ts - TAP_JUDGE_THRESHOLD..params.ts + params.dur, // TODO: check
                        raw_note: note,
                    },
                    MaterializedNote::SlideTrack(params) => {
                        let groups = params
                            .groups
                            .iter()
                            .map(|group| {
                                let segments = group
                                    .segments
                                    .iter()
                                    .flat_map(materialized_to_normalized_slide_segment)
                                    .collect();
                                NormalizedSlideSegmentGroup { segments }
                            })
                            .collect();
                        let mut path = SLIDE_PATH_GETTER
                            .get(NormalizedSlideTrack { groups })
                            .into_iter()
                            .flatten()
                            .flatten()
                            .collect::<Vec<_>>();
                        path.sort();
                        path.dedup();
                        let dur = params
                            .groups
                            .iter()
                            .map(|group| group.dur)
                            .sum::<DurationInSeconds>();
                        Note {
                            sensors: path,
                            dur: params.ts - SLIDE_JUDGE_THRESHOLD..params.ts + dur, // TODO: check
                            raw_note: note,
                        }
                    }
                })
                .collect::<Vec<_>>();

            let get_sensor_index = |sensor: &TouchSensor| match sensor.group() {
                Some('A') => sensor.index().unwrap(),
                Some('B') => sensor.index().unwrap() + 8,
                Some('C') => 16,
                Some('D') => sensor.index().unwrap() + 17,
                Some('E') => sensor.index().unwrap() + 25,
                _ => unreachable!(),
            };
            notes.sort_by(|a, b| a.dur.start.partial_cmp(&b.dur.start).unwrap());
            let mut groups: Vec<(TimestampInSeconds, Vec<Note>)> = notes
                .iter()
                .map(|note| (note.dur.end, vec![note.clone()]))
                .collect();
            let mut last_note_index: Vec<Option<usize>> = vec![None; 33];
            for (index, note) in notes.iter().enumerate() {
                note.sensors.iter().for_each(|sensor| {
                    let sensor_index = get_sensor_index(sensor) as usize;
                    if let Some(last_index) = last_note_index[sensor_index] {
                        if last_index != index
                            && note.dur.start < groups[last_index].0 + GROUP_DUR_THRESHOLD
                        {
                            // TODO: it can't work because of borrow checker:
                            // groups[index].1.extend(groups[last_index].1.drain(..));
                            let mut tmp = Vec::new();
                            std::mem::swap(&mut tmp, &mut groups[last_index].1);
                            groups[index].1.extend(tmp);
                            groups[index].0 = groups[index].0.max(groups[last_index].0);
                            groups[last_index].0 = TimestampInSeconds::NEG_INFINITY;
                        }
                    }
                });
                note.sensors.iter().for_each(|sensor| {
                    let sensor_index = get_sensor_index(sensor) as usize;
                    last_note_index[sensor_index] = Some(index);
                });
            }

            groups.retain(|group| !group.1.is_empty());
            groups.iter_mut().for_each(|group| {
                group
                    .1
                    .sort_by(|a, b| a.dur.start.partial_cmp(&b.dur.start).unwrap())
            });

            BeatmapData {
                maidata,
                diff,
                groups,
            }
        })
        .collect()
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}
