use bevy::prelude::*;

use crate::{spells::instances::FireballBundle, state::ServerState};

use common::{
    effects::{DamageEffect, EffectMarker},
    spells::{
        instances::FireballEffect, SpellMarker, SpellPlugin as CommonSpellPlugin, SpellTrigger,
    },
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
                let fireball_bundle = spell_entity_mut
                    .remove_bundle::<FireballBundle>()
                    .expect("fireball bundle not found");

                let fireball_effect = FireballEffect {
                    marker: EffectMarker,
                    target: fireball_bundle.common.target().into(),
                    damage: DamageEffect { amount: 30 },
                };
                world.spawn().insert_bundle(fireball_effect);
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
