use super::slide_data::{
    HitAreaData, BEND_L_DATA, CIRCLE_L_DATA, CORNER_DATA, CURVE_L_DATA, FAN_DATA, SKIP_L_DATA,
    STRAIGHT_DATA, THUNDER_L_DATA,
};
use crate::insn::{Key, TouchSensor};
use crate::transform;
use crate::transform::{NormalizedSlideSegment, NormalizedSlideSegmentShape, NormalizedSlideTrack};
use lazy_static::lazy_static;
use transform::transform::{Transformable, Transformer};

use enum_map::EnumMap;

#[derive(Clone, Debug)]
pub struct HitArea {
    pub hit_points: Vec<TouchSensor>,
    pub push_distance: f64,
    pub release_distance: f64,
}
pub type HitAreas = Vec<HitArea>;

pub fn hit_areas_to_path(hit_areas: HitAreas) -> Vec<Vec<TouchSensor>> {
    hit_areas
        .into_iter()
        .map(|hit_area| hit_area.hit_points)
        .collect()
}

pub struct SlideDataGetter {
    slide_hit_areas_list: EnumMap<NormalizedSlideSegmentShape, [[Option<HitAreas>; 8]; 8]>,
}

impl SlideDataGetter {
    fn add_data(
        &mut self,
        shape: NormalizedSlideSegmentShape,
        start: u8,
        destination: u8,
        hit_areas: HitAreas,
    ) {
        self.slide_hit_areas_list[shape][start as usize][destination as usize] = Some(hit_areas);
    }

    fn add_shape_data(
        &mut self,
        shape: NormalizedSlideSegmentShape,
        data: &[&[HitAreaData]; 8],
        flip: bool,
    ) {
        for (destination, hit_areas) in data.iter().enumerate() {
            if hit_areas.is_empty() {
                continue;
            }
            for rotation in 0..8 {
                let transformer = Transformer { rotation, flip };
                let start = Key::new(0).unwrap().transform(transformer).index();
                let destination = Key::new(destination as u8)
                    .unwrap()
                    .transform(transformer)
                    .index();
                let hit_areas = hit_areas
                    .iter()
                    .map(|hit_area_data| {
                        let hit_points = hit_area_data
                            .hit_points
                            .iter()
                            .map(|sensor| sensor.transform(transformer))
                            .collect();
                        HitArea {
                            hit_points,
                            push_distance: hit_area_data.push_distance,
                            release_distance: hit_area_data.release_distance,
                        }
                    })
                    .collect();
                self.add_data(shape, start, destination, hit_areas);
            }
        }
    }

    fn new() -> Self {
        let mut result = Self {
            slide_hit_areas_list: EnumMap::default(),
        };
        result.add_shape_data(NormalizedSlideSegmentShape::Straight, &STRAIGHT_DATA, false);
        result.add_shape_data(NormalizedSlideSegmentShape::CircleL, &CIRCLE_L_DATA, false);
        result.add_shape_data(NormalizedSlideSegmentShape::CircleR, &CIRCLE_L_DATA, true);
        result.add_shape_data(NormalizedSlideSegmentShape::CurveL, &CURVE_L_DATA, false);
        result.add_shape_data(NormalizedSlideSegmentShape::CurveR, &CURVE_L_DATA, true);
        result.add_shape_data(
            NormalizedSlideSegmentShape::ThunderL,
            &THUNDER_L_DATA,
            false,
        );
        result.add_shape_data(NormalizedSlideSegmentShape::ThunderR, &THUNDER_L_DATA, true);
        result.add_shape_data(NormalizedSlideSegmentShape::Corner, &CORNER_DATA, false);
        result.add_shape_data(NormalizedSlideSegmentShape::BendL, &BEND_L_DATA, false);
        result.add_shape_data(NormalizedSlideSegmentShape::BendR, &BEND_L_DATA, true);
        result.add_shape_data(NormalizedSlideSegmentShape::SkipL, &SKIP_L_DATA, false);
        result.add_shape_data(NormalizedSlideSegmentShape::SkipR, &SKIP_L_DATA, true);
        result.add_shape_data(NormalizedSlideSegmentShape::Fan, &FAN_DATA, false);
        result
    }

    pub fn get_by_segment(&self, segment: &NormalizedSlideSegment) -> Option<HitAreas> {
        self.slide_hit_areas_list[segment.shape()][segment.params().start.index() as usize]
            [segment.params().destination.index() as usize]
            .clone()
    }

    // pay attention to Fan Slide
    pub fn get(&self, track: &NormalizedSlideTrack) -> Option<HitAreas> {
        let mut result = HitAreas::new();
        for group in &track.groups {
            for segment in &group.segments {
                let data = self.get_by_segment(segment)?;
                assert!(!data.is_empty());
                if !result.is_empty() {
                    // Ensure the end is in the A TouchSensor
                    assert!(result.last().unwrap().hit_points.len() == 1);
                    // assert!(result.last().unwrap() == path.first().unwrap());
                    result.pop();
                }
                result.extend(data);
            }
        }
        Some(result)
    }

    pub fn get_path_by_segment(
        &self,
        segment: &NormalizedSlideSegment,
    ) -> Option<Vec<Vec<TouchSensor>>> {
        self.get_by_segment(segment).map(hit_areas_to_path)
    }

    pub fn get_path(&self, track: &NormalizedSlideTrack) -> Option<Vec<Vec<TouchSensor>>> {
        self.get(track).map(hit_areas_to_path)
    }
}

lazy_static! {
    pub static ref SLIDE_DATA_GETTER: SlideDataGetter = SlideDataGetter::new();
}
