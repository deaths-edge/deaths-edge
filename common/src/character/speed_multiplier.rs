pub struct SpeedMultiplier(pub f32);

impl SpeedMultiplier {
    const BASE_SPEED: f32 = 300.;

    pub fn speed(&self) -> f32 {
        Self::BASE_SPEED * self.0
    }
}
