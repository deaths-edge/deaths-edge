use bevy::utils::Instant;

#[derive(Default)]
pub struct InteruptState {
    interupted_until: Option<Instant>,
}

impl InteruptState {
    pub fn interupt_until(&mut self, instant: Instant) -> &mut Self {
        self.interupted_until = Some(instant);
        self
    }
}
