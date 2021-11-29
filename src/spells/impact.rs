use bevy::{app::Events, prelude::*};

use crate::effects::{DamageEffect, EffectMarker, EffectTarget};

use super::{
    instances::{FireballBundle, FireballEffect},
    SpellMarker,
};

pub struct SpellImpactEvent {
    pub id: Entity,
    pub spell_marker: SpellMarker,
}

impl SpellImpactEvent {
    pub fn trigger(&self, world: &mut World) {
        use SpellMarker::*;
        match self.spell_marker {
            Fireball => {
                let mut spell_entity_mut = world.entity_mut(self.id);
                let fireball_bundle = spell_entity_mut
                    .remove_bundle::<FireballBundle>()
                    .expect("fireball bundle not found");

                let fireball_effect = FireballEffect {
                    marker: EffectMarker,
                    target: EffectTarget::from(fireball_bundle.target),
                    damage: DamageEffect { amount: 30 },
                };
                world.spawn().insert_bundle(fireball_effect);
            }
        }
    }
}

pub fn spell_impact_system(world: &mut World) {
    // Drain all spell impact events
    let mut spell_impact_events = world
        .get_resource_mut::<Events<SpellImpactEvent>>()
        .expect("missing spell impact events");
    let spell_impact_events: Vec<_> = spell_impact_events.drain().collect();

    for spell_impact_event in spell_impact_events {
        spell_impact_event.trigger(world);
    }
}
