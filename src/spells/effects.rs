use bevy::prelude::*;

use crate::character::{CharacterHealth, CharacterIndex};

use super::SpellTarget;

pub struct EffectMarker;

pub struct DamageEffect {
    pub amount: u32,
}

pub struct HealEffect {
    pub amount: u32,
}

pub fn damage_effect_apply(
    // damage_query: Query<(&Entity, &DamageEffect, &SpellTarget)>,
    damage_query: Query<(Entity, &DamageEffect), With<EffectMarker>>,
    char_query: Query<(&CharacterIndex, &mut CharacterHealth)>,
    mut commands: Commands,
) {
    for (damage_entity, damage_effect) in damage_query.iter() {
        commands.entity(damage_entity).remove::<DamageEffect>();
        tracing::info!("took damage");
    }
}
