use super::{JudgeNote, JudgeType, OnSensorResult, Timing, TouchSensorStates, JUDGE_DATA};
use crate::insn::TouchSensor;
use crate::materialize::MaterializedTouchHold;

#[derive(Clone, Debug)]
pub struct TouchHold {
    pub sensor: TouchSensor,
    pub appear_time: f32,
    pub tail_time: f32,

    head_judge_type: JudgeType,
    head_result: Option<Timing>,
    prev_state: Option<bool>,
    prev_time: Option<f32>,
    release_time: f32,

    result: Option<Timing>,
}

impl From<MaterializedTouchHold> for TouchHold {
    fn from(m: MaterializedTouchHold) -> Self {
        Self {
            appear_time: m.ts,
            tail_time: m.ts + m.dur,
            sensor: m.sensor,
            head_judge_type: JudgeType::Touch,
            head_result: None,
            prev_state: None,
            prev_time: None,
            release_time: 0.0,
            result: None,
        }
    }
}

impl JudgeNote for TouchHold {
    fn get_start_time(&self) -> f32 {
        self.appear_time + JUDGE_DATA.judge_param(self.head_judge_type).as_ref()[Timing::TooFast]
    }

    fn get_end_time(&self) -> f32 {
        f32::max(
            self.appear_time
                + JUDGE_DATA.judge_param(self.head_judge_type).as_ref()[Timing::TooLate],
            self.tail_time,
        )
    }

    fn get_sensor(&self) -> Option<TouchSensor> {
        Some(self.sensor)
    }

    fn on_sensor(&mut self, current_time: f32) -> OnSensorResult {
        assert!(self.result.is_none());
        if current_time < self.get_start_time() {
            return OnSensorResult::TooFast;
        }
        self.head_result =
            Some(JUDGE_DATA.get_timing(self.head_judge_type, current_time - self.appear_time));
        if self.head_result != Some(Timing::TooLate) {
            OnSensorResult::Consumed
        } else {
            OnSensorResult::TooLate
        }
    }

    fn judge(&mut self, simulator: &TouchSensorStates, current_time: f32) {
        assert!(self.result.is_none());
        let curr_state = simulator.sensor_is_on(self.sensor);
        if current_time < self.appear_time + JUDGE_DATA.judge_touch_hold_head_s() {
            self.prev_state = Some(curr_state);
            self.prev_time = Some(current_time);
            return;
        }
        assert!(self.prev_state.is_some());
        let prev_state = self.prev_state.unwrap();
        let prev_time = self.prev_time.unwrap();
        if !prev_state
            && self.appear_time + JUDGE_DATA.judge_touch_hold_head_s()
                <= self.tail_time - JUDGE_DATA.judge_touch_hold_tail_s()
        {
            self.release_time += f32::min(
                current_time,
                self.tail_time - JUDGE_DATA.judge_touch_hold_tail_s(),
            ) - f32::max(
                prev_time,
                self.appear_time + JUDGE_DATA.judge_touch_hold_head_s(),
            );
        }
        if current_time >= self.get_end_time() {
            self.result = Some(JUDGE_DATA.get_hold_timing(
                self.tail_time - self.appear_time,
                self.release_time,
                self.head_result.unwrap_or(Timing::TooLate),
                true,
            ));
        }
    }

    fn get_judge_result(&self) -> Option<Timing> {
        self.result
    }
}
