use super::note::{get_all_sensors, JudgeNote, Note, TouchSensorStates};
use crate::insn::TouchSensor;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct MaiSimulator {
    sensor_states: TouchSensorStates,

    notes: Vec<Note>,
    notes_judge_on: HashMap<TouchSensor, VecDeque<usize>>,
    notes_judge_change: Vec<usize>,
    note_is_judged: Vec<bool>,
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

        if matches!(
            note,
            Note::Hold(_) | Note::TouchHold(_) | Note::Slide(_) | Note::FanSlide(_)
        ) {
            self.notes_judge_change.push(self.notes.len());
        }

        self.notes.push(note);
        self.note_is_judged.push(false);
    }

    fn update_sensor_change(&mut self, current_time: f32) {
        for i in (0..self.notes_judge_change.len()).rev() {
            let note_index = self.notes_judge_change[i];
            let note = &mut self.notes[note_index];
            note.judge(&self.sensor_states, current_time);
            if note.get_judge_result().is_some() {
                self.note_is_judged[note_index] = true;
                self.notes_judge_change.swap_remove(i);
            }
        }
    }

    // return true if sensor turns on
    pub fn change_sensor(&mut self, sensor: TouchSensor, current_time: f32) -> bool {
        if !self.sensor_states.sensor_is_on(sensor) {
            self.sensor_states.activate_sensor(sensor);
            let notes_judge_on = self.notes_judge_on.get_mut(&sensor).unwrap();
            while let Some(&note_index) = notes_judge_on.front() {
                let note = &mut self.notes[note_index];
                let consumed = if note.is_too_late(current_time) {
                    note.judge(&self.sensor_states, current_time);
                    false
                } else {
                    note.on_sensor(current_time)
                };
                if note.get_judge_result().is_some() {
                    self.note_is_judged[note_index] = true;
                    notes_judge_on.pop_front();
                }
                if consumed {
                    break;
                }
            }
        } else {
            self.sensor_states.deactivate_sensor(sensor);
        }
        self.update_sensor_change(current_time);
        self.sensor_states.sensor_is_on(sensor)
    }

    pub fn print_judge_result(&mut self) {
        for notes_judge_on in self.notes_judge_on.values_mut() {
            while let Some(&note_index) = notes_judge_on.front() {
                let note = &mut self.notes[note_index];
                note.judge(&self.sensor_states, f32::INFINITY);
                assert!(note.get_judge_result().is_some());
                self.note_is_judged[note_index] = true;
                notes_judge_on.pop_front();
            }
        }
        self.update_sensor_change(f32::INFINITY);

        assert!(self
            .notes_judge_on
            .values()
            .all(|notes_judge_on| notes_judge_on.is_empty()));
        assert!(self.notes_judge_change.is_empty());
        assert!(self.note_is_judged.iter().all(|&is_judged| is_judged));

        for note in &self.notes {
            println!("{:?}", note.get_judge_result().unwrap());
        }
    }
}

impl Default for MaiSimulator {
    fn default() -> Self {
        Self::new()
    }
}
