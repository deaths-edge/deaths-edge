use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct CharacterHealth {
    pub current: u32,
    pub total: u32,
}

impl CharacterHealth {
    pub fn apply_damage(&mut self, damage: u32) -> &mut Self {
        self.current = self.current.saturating_sub(damage);
        self
    }

    pub fn apply_heal(&mut self, heal: u32) -> &mut Self {
        self.current = self.total.min(self.current + heal);
        self
    }
}
