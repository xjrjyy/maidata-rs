use super::slide_data::SLIDE_DATA;
use crate::insn::TouchSensor;
use crate::transform;
use crate::transform::{NormalizedSlideSegment, NormalizedSlideSegmentShape, NormalizedSlideTrack};
use lazy_static::lazy_static;
use transform::transform::{Transformable, Transformer};

use enum_map::EnumMap;

pub type HitPoint = Vec<TouchSensor>;
pub type SlidePath = Vec<HitPoint>;

pub struct SlidePathGetter {
    slide_paths: EnumMap<NormalizedSlideSegmentShape, [[Option<SlidePath>; 8]; 8]>,
}

impl SlidePathGetter {
    fn new() -> Self {
        let mut slide_paths: EnumMap<NormalizedSlideSegmentShape, [[Option<SlidePath>; 8]; 8]> =
            EnumMap::default();
        for (shape, raw_slide_path) in SLIDE_DATA.iter() {
            assert_eq!(shape.chars().nth(0), Some('1'));
            let destination = shape.chars().nth_back(0).unwrap().to_digit(10).unwrap() as u8 - 1;
            let shape = match &shape[1..shape.len() - 1] {
                "-" => NormalizedSlideSegmentShape::Straight,
                "<" => NormalizedSlideSegmentShape::CircleL,
                ">" => NormalizedSlideSegmentShape::CircleR,
                "p" => NormalizedSlideSegmentShape::CurveL,
                "q" => NormalizedSlideSegmentShape::CurveR,
                "s" => NormalizedSlideSegmentShape::ThunderL,
                "z" => NormalizedSlideSegmentShape::ThunderR,
                "v" => NormalizedSlideSegmentShape::Corner,
                "qq" => NormalizedSlideSegmentShape::BendL,
                "pp" => NormalizedSlideSegmentShape::BendR,
                "V7" => NormalizedSlideSegmentShape::SkipL,
                "V3" => NormalizedSlideSegmentShape::SkipR,
                "w" => NormalizedSlideSegmentShape::Fan,
                _ => panic!("Invalid shape: {}", shape),
            };
            let parse_touch_sensor = |s: &str| {
                TouchSensor::new(
                    s.chars().nth(0).unwrap(),
                    s[1..].parse::<u8>().map(|x| x - 1).ok(),
                )
                .unwrap()
            };
            let raw_slide_path: Vec<_> = raw_slide_path
                .iter()
                .map(|hit_point| hit_point.split('/').map(parse_touch_sensor).collect())
                .collect();
            for rotation in 0..8 {
                let transformer = Transformer {
                    rotation,
                    flip: false,
                };
                let transformed_slide_path: SlidePath = raw_slide_path
                    .iter()
                    .map(|hit_point: &HitPoint| {
                        hit_point
                            .iter()
                            .map(|sensor| sensor.transform(transformer))
                            .collect()
                    })
                    .collect();
                slide_paths[shape][rotation as usize][((destination + rotation) % 8) as usize] =
                    Some(transformed_slide_path);
            }
        }
        Self { slide_paths }
    }

    pub fn get_by_segment(&self, segment: &NormalizedSlideSegment) -> Option<SlidePath> {
        self.slide_paths[segment.shape()][segment.params().start.index() as usize]
            [segment.params().destination.index() as usize]
            .clone()
    }

    // pay attention to Fan Slide
    pub fn get(&self, track: &NormalizedSlideTrack) -> Option<SlidePath> {
        let mut result = SlidePath::new();
        for group in &track.groups {
            for segment in &group.segments {
                let path = self.get_by_segment(segment)?;
                assert!(!path.is_empty());
                if !result.is_empty() {
                    // Ensure the end is in the A TouchSensor
                    assert!(result.last().unwrap().len() == 1);
                    assert!(result.last().unwrap() == path.first().unwrap());
                    result.pop();
                }
                result.extend(path);
            }
        }
        Some(result)
    }
}

lazy_static! {
    pub static ref SLIDE_PATH_GETTER: SlidePathGetter = SlidePathGetter::new();
}
