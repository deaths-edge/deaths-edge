use bevy::{app::Events, ecs::world::EntityMut, prelude::*};

use crate::effects::{DamageEffect, EffectMarker};

use super::{
    instances::{FireballBundle, FireballEffect},
    SpellMarker,
};

pub struct SpellImpactEvent {
    pub id: Entity,
    pub spell_marker: SpellMarker,
}

pub trait SpellTrigger {
    fn trigger(this: &SpellImpactEvent, world: &mut World) {
        use SpellMarker::*;
        let mut spell_entity_mut = world.entity_mut(this.id);

        match this.spell_marker {
            Fireball => {
                let fireball_bundle = spell_entity_mut
                    .remove_bundle::<FireballBundle>()
                    .expect("fireball bundle not found");

                let fireball_effect = FireballEffect {
                    marker: EffectMarker,
                    target: fireball_bundle.target().into(),
                    damage: DamageEffect { amount: 30 },
                };
                world.spawn().insert_bundle(fireball_effect);
            }
        }
    }
}

impl SpellImpactEvent {
    pub fn trigger<Trigger>(&self, world: &mut World)
    where
        Trigger: SpellTrigger,
    {
        Trigger::trigger(self, world);
    }
}

pub fn spell_impact<Trigger>(world: &mut World)
where
    Trigger: SpellTrigger,
{
    // Drain all spell impact events
    let mut spell_impact_events = world
        .get_resource_mut::<Events<SpellImpactEvent>>()
        .expect("missing spell impact events");
    let spell_impact_events: Vec<_> = spell_impact_events.drain().collect();

    for spell_impact_event in spell_impact_events {
        spell_impact_event.trigger::<Trigger>(world);
    }
}
