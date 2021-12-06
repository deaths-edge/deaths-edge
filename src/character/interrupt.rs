use bevy::utils::Instant;

#[derive(Default)]
pub struct InterruptState {
    interrupted_until: Option<Instant>,
}

impl InterruptState {
    pub fn interrupt_until(&mut self, instant: Instant) -> &mut Self {
        self.interrupted_until = Some(instant);
        self
    }
}
