use bevy::prelude::*;

use crate::character::{CharacterControl, CharacterHealth, CharacterSpeedMultiplier};

pub enum BuffType {
    Buff,
    Debuff,
}

pub trait BuffInterface {
    fn name(&self) -> &str;

    fn buff_type(&self) -> BuffType;

    fn apply(
        &self,
        control: &mut CharacterControl,
        speed_modifier: &mut CharacterSpeedMultiplier,
        health: &mut CharacterHealth,
    ) {
    }

    fn tick(
        &self,
        time: &Time,
        control: &mut CharacterControl,
        speed_modifier: &mut CharacterSpeedMultiplier,
        health: &mut CharacterHealth,
    ) {
    }

    fn remove(
        &self,
        control: &mut CharacterControl,
        speed_modifier: &mut CharacterSpeedMultiplier,
        health: &mut CharacterHealth,
    ) {
    }
}
