use std::time::Duration;

use bevy::{prelude::*, utils::Instant};

pub const GLOBAL_COOLDOWN: Duration = Duration::from_secs(1);

#[derive(Default, Debug, Component)]
pub struct LastCastInstant(pub Option<Instant>);

impl LastCastInstant {
    // pub fn elapsed(&self, time: &Time) -> Duration {
    //     let last_update = time.last_update().expect("last input not found");

    //     last_update - self.0
    // }
}
