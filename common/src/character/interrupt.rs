use bevy::utils::Instant;

#[derive(Default)]
pub struct CharacterInterruptState {
    interrupted_until: Option<Instant>,
}

impl CharacterInterruptState {
    pub fn interrupt_until(&mut self, instant: Instant) -> &mut Self {
        self.interrupted_until = Some(instant);
        self
    }
}
