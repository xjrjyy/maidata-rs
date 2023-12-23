#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BpmParams {
    pub new_bpm: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BeatDivisorParams {
    NewDivisor(u32),
    NewAbsoluteDuration(f32),
}
