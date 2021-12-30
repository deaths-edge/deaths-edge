#[derive(Debug)]
pub struct CharacterHealth {
    pub current: f32,
    pub total: f32,
}

impl CharacterHealth {
    pub fn apply_damage(&mut self, damage: f32) -> &mut Self {
        self.current = (self.current - damage).max(0.);
        self
    }

    pub fn apply_heal(&mut self, heal: f32) -> &mut Self {
        self.current = self.total.min(self.current + heal);
        self
    }
}
