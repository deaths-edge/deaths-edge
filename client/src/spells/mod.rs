mod instances;
mod materials;
mod spawn;

use bevy::prelude::*;

use common::{
    character::CASTING_LABEL,
    spells::{
        instances::FireballEffect, SpellMarker, SpellPlugin as CommonSpellPlugin, SpellTrigger,
    },
};

pub use instances::*;
pub use materials::*;

use crate::state::ClientState;

use spawn::spawn_spells;

pub struct SpellPlugin;

pub const SPELLS_LABEL: &str = "spells";

struct ClientSpellTrigger;

impl SpellTrigger for ClientSpellTrigger {
    fn trigger(this: &common::spells::SpellImpactEvent, world: &mut World) {
        use SpellMarker::*;
        let mut spell_entity_mut = world.entity_mut(this.id);

        match this.spell_marker {
            Fireball => {
                let fireball_bundle = spell_entity_mut
                    .remove_bundle::<FireballBundle>()
                    .expect("fireball bundle not found");

                let fireball_effect = FireballEffect {
                    marker: common::effects::EffectMarker,
                    target: fireball_bundle.common.target().into(),
                    damage: common::effects::DamageEffect { amount: 30 },
                };
                world.spawn().insert_bundle(fireball_effect);
            }
        }
    }
}

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spawn_spells = SystemSet::on_update(ClientState::Arena)
            .label(SPELLS_LABEL)
            .after(CASTING_LABEL)
            .with_system(spawn_spells.system());
        app.init_resource::<SpellMaterials>()
            .add_plugin(CommonSpellPlugin::new(
                ClientState::Arena,
                ClientSpellTrigger,
            ))
            .add_system_set(spawn_spells);
    }
}
