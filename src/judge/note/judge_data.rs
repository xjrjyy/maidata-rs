use super::{JudgeParam, JudgeType, Timing};
use enum_map::EnumMap;
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct JudgeData {
    judge_adjust_s: f32,
    judge_hold_head_s: f32,
    judge_hold_tail_s: f32,
    judge_touch_hold_head_s: f32,
    judge_touch_hold_tail_s: f32,
    judge_param_table: EnumMap<JudgeType, JudgeParam>,
    hold_judge_percent: [i32; 5],
    hold_judge_param: [EnumMap<Timing, Timing>; 5],
}

impl JudgeData {
    fn new() -> Self {
        let judge_adjust_s = 0.05;
        Self {
            judge_adjust_s,
            judge_hold_head_s: 0.05 + judge_adjust_s,
            judge_hold_tail_s: 0.15 + judge_adjust_s,
            judge_touch_hold_head_s: 0.2 + judge_adjust_s,
            judge_touch_hold_tail_s: 0.15 + judge_adjust_s,
            judge_param_table: EnumMap::from_array([
                JudgeParam::new([
                    -9f32,
                    -6f32,
                    -5f32,
                    -4f32,
                    -3f32,
                    -2f32,
                    -1f32,
                    1f32,
                    2f32,
                    3f32,
                    4f32,
                    5f32,
                    6f32,
                    9f32,
                    f32::INFINITY,
                ]),
                JudgeParam::new([
                    -9f32,
                    -9f32,
                    -9f32,
                    -9f32,
                    -9f32,
                    -9f32,
                    -9f32,
                    9f32,
                    10.5f32,
                    12f32,
                    13f32,
                    14f32,
                    15f32,
                    18f32,
                    f32::INFINITY,
                ]),
                JudgeParam::new([
                    -36f32,
                    -26f32,
                    -22f32,
                    -18f32,
                    -14f32,
                    -14f32,
                    -14f32,
                    14f32,
                    14f32,
                    14f32,
                    16f32,
                    22f32,
                    26f32,
                    36f32,
                    f32::INFINITY,
                ]),
                JudgeParam::new([
                    -9f32,
                    -9f32,
                    -9f32,
                    -9f32,
                    -9f32,
                    -9f32,
                    -9f32,
                    9f32,
                    9f32,
                    9f32,
                    9f32,
                    9f32,
                    9f32,
                    9f32,
                    f32::INFINITY,
                ]),
            ]),
            hold_judge_percent: [0, 33, 67, 95, 100],
            hold_judge_param: [
                EnumMap::from_array([
                    Timing::FastGood,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastPerfect,
                    Timing::FastPerfect,
                    Timing::Critical,
                    Timing::LatePerfect,
                    Timing::LatePerfect,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGood,
                ]),
                EnumMap::from_array([
                    Timing::FastGood,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastPerfect,
                    Timing::FastPerfect,
                    Timing::LatePerfect,
                    Timing::LatePerfect,
                    Timing::LatePerfect,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGood,
                ]),
                EnumMap::from_array([
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::FastGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGreat,
                    Timing::LateGood,
                    Timing::LateGood,
                ]),
                EnumMap::from_array([
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                ]),
                EnumMap::from_array([
                    Timing::TooFast,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::FastGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::LateGood,
                    Timing::TooLate,
                ]),
            ],
        }
    }

    pub fn judge_adjust_s(&self) -> f32 {
        self.judge_adjust_s
    }

    pub fn judge_hold_head_s(&self) -> f32 {
        self.judge_hold_head_s
    }

    pub fn judge_hold_tail_s(&self) -> f32 {
        self.judge_hold_tail_s
    }

    pub fn judge_touch_hold_head_s(&self) -> f32 {
        self.judge_touch_hold_head_s
    }

    pub fn judge_touch_hold_tail_s(&self) -> f32 {
        self.judge_touch_hold_tail_s
    }

    pub fn judge_param(&self, judge_type: JudgeType) -> &JudgeParam {
        &self.judge_param_table[judge_type]
    }

    pub fn get_timing(&self, judge_type: JudgeType, delta_time: f32) -> Timing {
        let judge_timing = &self.judge_param_table[judge_type];
        judge_timing
            .judge_flame_list
            .iter()
            .find(|(_, &v)| delta_time < v)
            .map(|(k, _)| k)
            .unwrap()
    }

    pub fn get_hold_timing(
        &self,
        mut dur_time: f32,
        release_time: f32,
        head_reault: Timing,
        is_touch_hold: bool,
    ) -> Timing {
        dur_time -= if is_touch_hold {
            self.judge_touch_hold_head_s + self.judge_touch_hold_tail_s
        } else {
            self.judge_hold_head_s + self.judge_hold_tail_s
        };
        if dur_time < 0.0 {
            return head_reault;
        }
        // TODO: fix this
        assert!(release_time - f32::EPSILON <= dur_time);
        let release_percent = f32::ceil((release_time - f32::EPSILON) / dur_time * 100.0) as i32;
        for (i, &percent) in self.hold_judge_percent.iter().enumerate() {
            if release_percent <= percent {
                return self.hold_judge_param[i][head_reault];
            }
        }
        unreachable!();
    }
}

lazy_static! {
    pub static ref JUDGE_DATA: JudgeData = JudgeData::new();
}
