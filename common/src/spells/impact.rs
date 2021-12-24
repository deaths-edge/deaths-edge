use bevy::{app::Events, prelude::*};

use super::SpellMarker;

pub struct SpellImpactEvent {
    pub id: Entity,
    pub spell_marker: SpellMarker,
}

pub trait SpellTrigger {
    fn trigger(this: &SpellImpactEvent, world: &mut World);
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
