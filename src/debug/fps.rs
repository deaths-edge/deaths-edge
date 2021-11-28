use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, utils::Instant};

pub struct FrameCounter {
    pub frames: u64,
    pub last_flush: Option<Instant>,
    pub history: VecDeque<(&'static str, u64)>,
}

impl Default for FrameCounter {
    fn default() -> Self {
        Self {
            frames: 0,
            last_flush: None,
            history: std::iter::repeat(("", 0)).take(Self::LENGTH).collect(),
        }
    }
}

pub fn record_fps(time: Res<Time>, mut frames: ResMut<FrameCounter>) {
    if let Some(instant) = time.last_update() {
        frames.update(instant);
    }
}

impl FrameCounter {
    const LENGTH: usize = 30;
    const FLUSH_INTERVAL: Duration = Duration::from_secs(1);

    pub fn update(&mut self, time: Instant) {
        self.frames += 1;

        let delta = self
            .last_flush
            .and_then(|last_time| time.checked_duration_since(last_time));

        if let Some(delta) = delta {
            if delta > Self::FLUSH_INTERVAL {
                if self.history.len() < Self::LENGTH {
                    self.history.pop_front();
                }

                let fps = self.frames as f32 / delta.as_secs_f32();

                self.history.pop_front();
                self.history.push_back(("", fps as u64));

                self.frames = 0;
                self.last_flush = Some(time);
            }
        }

        // This ensures one slice
        self.history.make_contiguous();
    }

    pub fn history(&self) -> &[(&str, u64)] {
        self.history.as_slices().0 // Invariant
    }
}
