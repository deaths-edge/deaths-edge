use std::time::Duration;

use bevy::{core::Time, utils::Instant};

// pub struct Cooldown(Instant);

pub const GLOBAL_COOLDOWN: Duration = Duration::from_secs(1);

pub struct LastCastInstant(Instant);

impl From<Instant> for LastCastInstant {
    fn from(value: Instant) -> Self {
        Self(value)
    }
}

impl LastCastInstant {
    pub fn elapsed(&self, time: &Time) -> Duration {
        let last_update = time.last_update().expect("last input not found");

        last_update - self.0
    }
}
