use super::super::slide_data_getter::SLIDE_DATA_GETTER;
use super::{JudgeNote, JudgeType, Timing, TouchSensorStates, JUDGE_DATA};
use crate::insn::TouchSensor;
use crate::materialize::{MaterializedSlideSegment, MaterializedSlideTrack};
use crate::transform::{
    NormalizedSlideSegment, NormalizedSlideSegmentParams, NormalizedSlideSegmentShape,
    NormalizedSlideTrack,
};

#[derive(Clone, Debug)]
pub struct Slide {
    pub path: Vec<Vec<TouchSensor>>,
    pub appear_time: f64,
    pub tail_time: f64,
    pub _is_break: bool,

    judge_check_sensor_1: bool,
    judge_check_sensor_3: bool,

    judge_type: JudgeType,
    pub judge_index: usize,
    pub judge_is_on: bool,
    pub judge_sub_sensor: Option<TouchSensor>,

    result: Option<Timing>,
}

impl TryFrom<MaterializedSlideTrack> for Slide {
    type Error = &'static str;

    fn try_from(m: MaterializedSlideTrack) -> Result<Self, Self::Error> {
        if m.segments
            .iter()
            .any(|segment| segment.shape == NormalizedSlideSegmentShape::Fan)
        {
            return Err("Fan Slide is not supported");
        }
        let dur = m.dur;
        let normalized_track = NormalizedSlideTrack {
            segments: m
                .segments
                .iter()
                .map(materialized_to_normalized_slide_segment)
                .collect::<Vec<_>>(),
        };

        // Why check head???
        let head_segment = normalized_track.segments[0];
        let head_is_thunder = head_segment.shape() == NormalizedSlideSegmentShape::ThunderL
            || head_segment.shape() == NormalizedSlideSegmentShape::ThunderR;
        let head_distance = (head_segment.params().destination.index() + 8
            - head_segment.params().start.index())
            % 8;

        Ok(Self {
            path: SLIDE_DATA_GETTER
                .get_path(&normalized_track)
                .ok_or("Slide path not found")?,
            appear_time: m.ts,
            tail_time: m.start_ts + dur,
            _is_break: m.is_break,
            judge_check_sensor_1: head_is_thunder,
            judge_check_sensor_3: head_is_thunder && head_distance == 4,
            judge_type: JudgeType::Slide,
            judge_index: 0,
            judge_is_on: false,
            judge_sub_sensor: None,
            result: None,
        })
    }
}

impl Slide {
    pub fn from_fan_single_segment(
        segment: &MaterializedSlideSegment,
        parent: &MaterializedSlideTrack,
    ) -> Result<Self, &'static str> {
        assert!(segment.shape == NormalizedSlideSegmentShape::Fan);
        Ok(Self {
            path: SLIDE_DATA_GETTER
                .get_path_by_segment(&materialized_to_normalized_slide_segment(segment))
                .ok_or("Slide path not found")?,
            appear_time: parent.ts,
            tail_time: parent.start_ts + parent.dur,
            _is_break: parent.is_break,
            judge_check_sensor_1: false,
            judge_check_sensor_3: false,
            judge_type: JudgeType::Slide,
            judge_index: 0,
            judge_is_on: false,
            judge_sub_sensor: None,
            result: None,
        })
    }
}

impl Slide {
    fn check_sensor(&mut self, simulator: &TouchSensorStates, index: usize, is_on: bool) -> bool {
        if index >= self.path.len() {
            return false;
        }
        if !is_on {
            for sensor in self.path[index].iter() {
                if simulator.sensor_is_on(*sensor) {
                    self.judge_index = index;
                    self.judge_is_on = true;
                    self.judge_sub_sensor = Some(*sensor);
                    if self.judge_index == self.path.len() - 1 {
                        self.judge_index = self.path.len();
                    }
                    return true;
                }
            }
        } else {
            assert!(index == self.judge_index && self.judge_is_on);
            if !simulator.sensor_is_on(self.judge_sub_sensor.unwrap()) {
                self.judge_index += 1;
                self.judge_is_on = false;
                self.judge_sub_sensor = None;
                return true;
            }
        }
        false
    }

    pub fn is_next_sensor_check(&self) -> bool {
        if self.judge_is_on {
            return true;
        }
        // TODO: Check if this is correct
        if self.judge_check_sensor_1 && self.judge_index == 1 {
            return false;
        }
        if self.judge_check_sensor_3 && self.judge_index == 3 {
            return false;
        }
        self.path.len() > 3 || self.judge_index + 1 != self.path.len() - 1
    }

    fn compute_judge_result(&self, current_time: f64) -> Option<Timing> {
        if self.judge_index < self.path.len() {
            return None;
        }
        // TODO: Fix Slide Critical timing (depends on slide wait time)
        let mut result = JUDGE_DATA.get_timing(self.judge_type, current_time - self.tail_time);
        if result == Timing::TooFast {
            result = Timing::FastGood;
        }
        Some(result)
    }
}

impl JudgeNote for Slide {
    fn get_start_time(&self) -> f64 {
        // TODO: check if this is correct
        self.appear_time + JUDGE_DATA.judge_param(JudgeType::Tap).as_ref()[Timing::FastGood]
    }

    fn get_end_time(&self) -> f64 {
        self.tail_time + JUDGE_DATA.judge_param(self.judge_type).as_ref()[Timing::LateGood]
    }

    fn judge(&mut self, simulator: &TouchSensorStates, current_time: f64) {
        assert!(self.result.is_none());
        // Do not judge if too late
        if self.is_too_late(current_time) {
            assert!(self.judge_index < self.path.len());
            self.result = Some(if self.judge_index + 1 == self.path.len() {
                Timing::LateGood
            } else {
                Timing::TooLate
            });
            return;
        }

        loop {
            let mut changed = self.check_sensor(simulator, self.judge_index, self.judge_is_on);
            if !changed && self.is_next_sensor_check() {
                changed = self.check_sensor(simulator, self.judge_index + 1, false);
            }
            if !changed || self.judge_index == self.path.len() {
                break;
            }
        }
        if self.judge_index == self.path.len() {
            self.result = self.compute_judge_result(current_time);
            assert!(self.result.is_some());
        }
    }

    fn get_judge_result(&self) -> Option<Timing> {
        self.result
    }
}

fn materialized_to_normalized_slide_segment(
    segment: &MaterializedSlideSegment,
) -> NormalizedSlideSegment {
    NormalizedSlideSegment::new(
        segment.shape,
        NormalizedSlideSegmentParams {
            start: segment.start,
            destination: segment.destination,
        },
    )
}
