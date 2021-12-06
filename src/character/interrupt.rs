use bevy::{prelude::*, utils::Instant};

#[derive(Debug, Default, Component)]
pub struct InterruptState {
    interrupted_until: Option<Instant>,
}

impl InterruptState {
    pub fn interrupt_until(&mut self, instant: Instant) -> &mut Self {
        self.interrupted_until = Some(instant);
        self
    }
}
