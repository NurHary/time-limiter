use rodio::*;
use std::fs::File;
use std::time::{Duration, SystemTime};

pub enum TimeMode {
    Play,
    Pause,
    Stop,
}

pub enum Model {
    Pomodoro,
    Stopwatch,
    Timer,
}

pub struct SlotModel {
    pub model: Model,
    pub duration: Duration,
}

impl SlotModel {
    pub fn new(model: Model, duration: Duration) -> Self {
        Self { model, duration }
    }
}

impl Default for SlotModel {
    fn default() -> Self {
        Self {
            model: Model::Pomodoro,
            duration: Duration::from_secs(1500),
        }
    }
}

pub struct TimeSlot {
    // ini adalah tempat untuk
    pub mode: TimeMode,
    pub model: SlotModel,
}

impl Default for TimeSlot {
    fn default() -> Self {
        Self {
            mode: TimeMode::Stop,
            model: SlotModel::default(),
        }
    }
}
