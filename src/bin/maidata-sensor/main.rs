use maidata::container::{AssociatedBeatmapData, Maidata};
use std::time::Instant;
use walkdir::WalkDir;

#[warn(dead_code)]
struct BeatmapData<'a> {
    maidata: &'a Maidata,
    diff: AssociatedBeatmapData<'a>,
    groups: Vec<Vec<Note>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::args().nth(1).expect("usage: $0 <path/to/charts>");

    let start = Instant::now();

    let maidata_vec = WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir() && e.file_name() == "maidata.txt")
        .map(|e| read_file(e.path()))
        .map(|content| {
            let (maidata, state) = maidata::container::lex_maidata(&content);
            assert!(!state.has_messages());
            maidata
        })
        .collect::<Vec<_>>();
    let beatmap_data_vec = maidata_vec
        .iter()
        .flat_map(|maidata| {
            maidata.iter_difficulties().map(move |diff| {
                parse_maidata(&diff).map(|groups| BeatmapData {
                    maidata,
                    diff,
                    groups,
                })
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut result = beatmap_data_vec
        .iter()
        .map(|data| {
            let max_slide_duration = data
                .groups
                .iter()
                .map(|group| {
                    group
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
            (data, max_slide_duration)
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (data, max_slide_duration) in result.iter().take(20) {
        println!(
            "{:.2}s: {} [{:?}]",
            max_slide_duration,
            data.maidata.title(),
            data.diff.difficulty()
        );
    }

    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    Ok(())
}

use maidata::insn::{Key, TouchSensor};
use maidata::judge::slide_path::SLIDE_PATH_GETTER;
use maidata::materialize::{DurationInSeconds, Note as MaterializedNote, TimestampInSeconds};
use maidata::transform::transform::{Transformable, Transformer};
use maidata::transform::{
    NormalizedSlideSegment, NormalizedSlideSegmentGroup, NormalizedSlideSegmentParams,
    NormalizedSlideSegmentShape, NormalizedSlideTrack,
};
#[allow(unused_imports)]
use maidata::Level;

const FRAMES_PER_SECOND: DurationInSeconds = 60.0;
const FRAME_DURATION: DurationInSeconds = 1.0 / FRAMES_PER_SECOND;

const TAP_JUDGE_THRESHOLD: DurationInSeconds = FRAME_DURATION * 9.0;
const SLIDE_JUDGE_THRESHOLD: DurationInSeconds = FRAME_DURATION * 6.0;

const GROUP_DUR_THRESHOLD: DurationInSeconds = 0.2;

#[derive(Clone, Debug)]
struct Note {
    sensors: Vec<TouchSensor>,
    dur: std::ops::Range<TimestampInSeconds>,
    raw_note: MaterializedNote,
}

fn key_to_sensor(key: Key) -> TouchSensor {
    TouchSensor::new('A', Some(key.index())).unwrap()
}

#[rustfmt::skip]
fn materialized_to_normalized_slide_segment(
    segment: &maidata::materialize::MaterializedSlideSegment,
) -> NormalizedSlideSegment {
    let normalized_params = NormalizedSlideSegmentParams {
        start: segment.start,
        destination: segment.destination,
    };
    match segment.shape {
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
        NormalizedSlideSegmentShape::Fan => panic!("Fan shape is not supported"),
    }
}

fn parse_maidata(diff: &AssociatedBeatmapData) -> Option<Vec<Vec<Note>>> {
    // if !match diff.level() {
    //     Some(Level::Normal(level)) => (11..=13).contains(&level),
    //     Some(Level::Plus(level)) => (11..=13).contains(&level),
    //     Some(Level::Char(_)) => false,
    //     None => false,
    // } {
    //     return None;
    // }

    let mut mcx = maidata::materialize::MaterializationContext::with_offset(0.0);
    let notes = mcx.materialize_insns(diff.iter_insns());

    let mut notes = notes
        .into_iter()
        .map(|note| (*note).clone())
        .map(|note| match &note {
            MaterializedNote::Bpm(_) => todo!(),
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
                let mut path = if params.groups.iter().any(|group| {
                    group
                        .segments
                        .iter()
                        .any(|segment| segment.shape == NormalizedSlideSegmentShape::Fan)
                }) {
                    assert!(params.groups.len() == 1 && params.groups[0].segments.len() == 1);
                    let segment = &params.groups[0].segments[0];
                    // TODO: handle fan slide
                    [
                        NormalizedSlideSegment::Fan(NormalizedSlideSegmentParams {
                            start: segment.start,
                            destination: segment.destination.transform(Transformer {
                                rotation: 7,
                                flip: false,
                            }),
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
                            }),
                        }),
                    ]
                    .iter()
                    .flat_map(|segment| {
                        SLIDE_PATH_GETTER
                            .get(&NormalizedSlideTrack {
                                groups: vec![NormalizedSlideSegmentGroup {
                                    segments: vec![*segment],
                                }],
                            })
                            .into_iter()
                            .flatten()
                            .flatten()
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
                } else {
                    let groups = params
                        .groups
                        .iter()
                        .map(|group| {
                            let segments = group
                                .segments
                                .iter()
                                .map(materialized_to_normalized_slide_segment)
                                .collect();
                            NormalizedSlideSegmentGroup { segments }
                        })
                        .collect();
                    SLIDE_PATH_GETTER
                        .get(&NormalizedSlideTrack { groups })
                        .into_iter()
                        .flatten()
                        .flatten()
                        .collect::<Vec<_>>()
                };
                path.sort();
                path.dedup();
                let dur = params
                    .groups
                    .iter()
                    .map(|group| group.dur)
                    .sum::<DurationInSeconds>();
                Note {
                    sensors: path,
                    dur: params.ts - SLIDE_JUDGE_THRESHOLD..params.start_ts + dur, // TODO: check
                    raw_note: note,
                }
            }
        })
        .collect::<Vec<_>>();

    let get_sensor_index = |sensor: &TouchSensor| match sensor.group() {
        'A' => sensor.index().unwrap(),
        'B' => sensor.index().unwrap() + 8,
        'C' => 16,
        'D' => sensor.index().unwrap() + 17,
        'E' => sensor.index().unwrap() + 25,
        _ => unreachable!(),
    };
    notes.sort_by(|a, b| a.dur.start.partial_cmp(&b.dur.start).unwrap());

    let find = |parent: &mut Vec<usize>, mut x: usize| -> usize {
        let mut y = x;
        while parent[y] != y {
            y = parent[y];
        }
        while parent[x] != x {
            let z = parent[x];
            parent[x] = y;
            x = z;
        }
        y
    };
    let union = |parent: &mut Vec<usize>, x: usize, y: usize| {
        let x = find(parent, x);
        let y = find(parent, y);
        parent[x] = y;
    };
    let mut parent = (0..notes.len()).collect::<Vec<_>>();
    let mut sensor_info = [(0, TimestampInSeconds::NEG_INFINITY); 33];
    for (index, note) in notes.iter().enumerate() {
        note.sensors.iter().for_each(|sensor| {
            let sensor_index = get_sensor_index(sensor) as usize;
            let (last_index, end_ts) = sensor_info.get_mut(sensor_index).unwrap();
            if note.dur.start < *end_ts + GROUP_DUR_THRESHOLD {
                union(&mut parent, *last_index, index);
            }
            *last_index = index;
            *end_ts = end_ts.max(note.dur.end);
        });
    }

    let mut groups = vec![Vec::new(); notes.len()];
    for (index, note) in notes.iter().enumerate() {
        let group = find(&mut parent, index);
        groups[group].push(note.clone());
    }
    Some(
        groups
            .into_iter()
            .filter(|group| !group.is_empty())
            .collect(),
    )
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}
