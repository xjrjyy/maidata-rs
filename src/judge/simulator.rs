use super::note::{get_all_sensors, JudgeNote, Note, Timing, TouchSensorStates};
use crate::{insn::TouchSensor, judge::note::OnSensorResult};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct MaiSimulator {
    sensor_states: TouchSensorStates,

    pub notes: Vec<Note>,
    notes_judge_on: HashMap<TouchSensor, VecDeque<usize>>,
    notes_judge_change: Vec<usize>,
    pub note_is_judged: Vec<bool>,

    worst_judge_result: Option<Timing>,
}

pub fn worse_judge_result(lhs: Timing, rhs: Timing) -> Timing {
    let rank_lhs = lhs as i32 - Timing::Critical as i32;
    let rank_rhs = rhs as i32 - Timing::Critical as i32;
    if i32::abs(rank_lhs) > i32::abs(rank_rhs) {
        lhs
    } else {
        rhs
    }
}

impl MaiSimulator {
    pub fn new() -> Self {
        Self {
            sensor_states: TouchSensorStates::new(),
            notes: Vec::new(),
            notes_judge_on: get_all_sensors()
                .into_iter()
                .map(|sensor| (sensor, VecDeque::new()))
                .collect(),
            notes_judge_change: Vec::new(),
            note_is_judged: Vec::new(),
            worst_judge_result: None,
        }
    }

    pub fn get_worst_judge_result(&self) -> Option<Timing> {
        self.worst_judge_result
    }

    pub fn update_too_late(&mut self, current_time: f32) {
        for note_index in 0..self.notes.len() {
            if self.note_is_judged[note_index] {
                continue;
            }
            let note = &mut self.notes[note_index];
            if note.get_end_time() <= current_time {
                note.judge(&self.sensor_states, current_time);
                assert!(note.get_judge_result().is_some());
                self.add_judged_note(note_index);
            }
        }
    }

    fn add_judged_note(&mut self, note_index: usize) {
        assert!(!self.note_is_judged[note_index]);
        self.note_is_judged[note_index] = true;
        let note = &self.notes[note_index];
        if let Some(judge_result) = note.get_judge_result() {
            self.worst_judge_result = match self.worst_judge_result {
                Some(worst_judge_result) => {
                    Some(worse_judge_result(worst_judge_result, judge_result))
                }
                None => Some(judge_result),
            };
        }
    }

    // TODO: note should be sorted by start time
    pub fn add_note(&mut self, note: Note) {
        if let Some(sensor) = note.get_sensor() {
            let notes_judge_on = self.notes_judge_on.get_mut(&sensor).unwrap();
            if let Some(&last_note_index) = notes_judge_on.back() {
                let last_note = &self.notes[last_note_index];
                if last_note.get_start_time() > note.get_start_time() {
                    panic!("note's start time is earlier than last note's start time");
                }
            }
            notes_judge_on.push_back(self.notes.len());
        }

        self.notes.push(note);
        self.note_is_judged.push(false);

        if matches!(
            &self.notes.last().unwrap(),
            &Note::Hold(_) | &Note::TouchHold(_) | &Note::Slide(_) | &Note::FanSlide(_)
        ) {
            self.notes_judge_change.push(self.notes.len() - 1);
            // TODO: fix this
            self.update_sensor_change(self.notes.last().unwrap().get_start_time());
        }
    }

    fn update_sensor_change(&mut self, current_time: f32) {
        for i in (0..self.notes_judge_change.len()).rev() {
            let note_index = self.notes_judge_change[i];
            if self.note_is_judged[note_index] {
                self.notes_judge_change.swap_remove(i);
                continue;
            }
            let note = &mut self.notes[note_index];
            note.judge(&self.sensor_states, current_time);
            if note.get_judge_result().is_some() {
                self.notes_judge_change.swap_remove(i);
                self.add_judged_note(note_index);
            }
        }
    }

    // return true if sensor turns on
    pub fn change_sensor(&mut self, sensor: TouchSensor, current_time: f32) -> bool {
        if !self.sensor_states.sensor_is_on(sensor) {
            self.sensor_states.activate_sensor(sensor);
            // TODO: check if this is correct
            while let Some(&note_index) = self.notes_judge_on.get_mut(&sensor).unwrap().front() {
                let note = &mut self.notes[note_index];
                assert!(note.get_judge_result().is_none());
                match note.on_sensor(current_time) {
                    OnSensorResult::TooFast => {
                        break;
                    }
                    OnSensorResult::Consumed => {
                        self.notes_judge_on.get_mut(&sensor).unwrap().pop_front();
                        if note.get_judge_result().is_some() {
                            self.add_judged_note(note_index);
                        }
                        break;
                    }
                    OnSensorResult::TooLate => {
                        self.notes_judge_on.get_mut(&sensor).unwrap().pop_front();
                        note.judge(&self.sensor_states, current_time);
                        if note.get_judge_result().is_some() {
                            self.add_judged_note(note_index);
                        }
                    }
                }
            }
        } else {
            self.sensor_states.deactivate_sensor(sensor);
        }
        self.update_sensor_change(current_time);
        self.sensor_states.sensor_is_on(sensor)
    }

    pub fn finish(&mut self) {
        for note_index in 0..self.notes.len() {
            let note = &mut self.notes[note_index];
            if self.note_is_judged[note_index] {
                continue;
            }
            note.judge(&self.sensor_states, f32::INFINITY);
            assert!(note.get_judge_result().is_some());
            self.add_judged_note(note_index);
        }
        self.notes_judge_on
            .values_mut()
            .for_each(|notes_judge_on| notes_judge_on.clear());
        self.update_sensor_change(f32::INFINITY);

        assert!(self.notes_judge_change.is_empty());
        assert!(self.note_is_judged.iter().all(|&is_judged| is_judged));
    }

    pub fn print_judge_result(&mut self) {
        for note in &self.notes {
            println!("{:?}", note.get_judge_result());
        }
    }
}

impl Default for MaiSimulator {
    fn default() -> Self {
        Self::new()
    }
}
