use std::{ops::Deref, time::Duration};

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
    pub fn elapsed(&self, time: &Time) -> Option<Duration> {
        let last_update = time.last_update()?;

        last_update.checked_duration_since(self.0)
    }
}
