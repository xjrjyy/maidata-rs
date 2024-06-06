use super::slide::Slide;
use super::{JudgeNote, Timing, TouchSensorStates};
use crate::materialize::{MaterializedSlideSegment, MaterializedSlideTrack};
use crate::transform::NormalizedSlideSegmentShape;

// TODO: move to slide.rs
#[derive(Clone, Debug)]
pub struct FanSlide {
    pub sub_slides: Vec<Slide>,
}

impl TryFrom<MaterializedSlideTrack> for FanSlide {
    type Error = &'static str;

    fn try_from(m: MaterializedSlideTrack) -> Result<Self, Self::Error> {
        if m.groups.len() != 1 || m.groups[0].segments.len() != 1 {
            return Err("Fan Slide must have only one group and one segment");
        }
        let segment = m.groups[0].segments[0];
        if segment.shape != NormalizedSlideSegmentShape::Fan {
            return Err("Fan Slide must have a fan segment");
        }
        let sub_slides = [
            MaterializedSlideSegment {
                start: ((segment.start.index() + 7) % 8).try_into().unwrap(),
                destination: segment.destination,
                shape: NormalizedSlideSegmentShape::Fan,
            },
            MaterializedSlideSegment {
                start: segment.start,
                destination: segment.destination,
                shape: NormalizedSlideSegmentShape::Fan,
            },
            MaterializedSlideSegment {
                start: ((segment.start.index() + 1) % 8).try_into().unwrap(),
                destination: segment.destination,
                shape: NormalizedSlideSegmentShape::Fan,
            },
        ]
        .iter()
        .map(|segment| Slide::from_fan_single_segment(segment, &m))
        .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { sub_slides })
    }
}

impl JudgeNote for FanSlide {
    fn get_start_time(&self) -> f32 {
        self.sub_slides
            .iter()
            .map(|slide| slide.get_start_time())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    fn get_end_time(&self) -> f32 {
        self.sub_slides
            .iter()
            .map(|slide| slide.get_end_time())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    fn judge(&mut self, getter: &TouchSensorStates, current_time: f32) {
        for slide in &mut self.sub_slides {
            slide.judge(getter, current_time);
        }
    }

    fn get_judge_result(&self) -> Option<Timing> {
        if self
            .sub_slides
            .iter()
            .any(|slide| slide.get_judge_result().is_none())
        {
            return None;
        }
        self.sub_slides
            .iter()
            .map(|slide| slide.get_judge_result().unwrap())
            .max()
    }
}
