use bevy::prelude::*;

use crate::{
    spells::instances::{FireballBundle, SpearBundle},
    state::ServerState,
};

use common::spells::{
    instances::ToEffect, SpellMarker, SpellPlugin as CommonSpellPlugin, SpellTrigger,
};

use spawn::spawn_spells;

pub mod instances;
mod spawn;

pub struct ServerSpellTrigger;

impl SpellTrigger for ServerSpellTrigger {
    fn trigger(this: &common::spells::SpellImpactEvent, world: &mut World) {
        use SpellMarker::*;
        let mut spell_entity_mut = world.entity_mut(this.id);

        match this.spell_marker {
            Fireball => {
                FireballBundle::process_spell(this.id, world);
            }
            Spear => {
                SpearBundle::process_spell(this.id, world);
            }
        }
    }
}

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CommonSpellPlugin::new(ServerState, ServerSpellTrigger))
            .add_system(spawn_spells.system());
    }
}
