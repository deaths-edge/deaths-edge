mod instances;
mod materials;

use bevy::prelude::*;

use common::{
    character::{CharacterMarker, CASTING_LABEL},
    spells::{
        instances::FireballBundle as CommonFireballBundle, Spell, SpellPlugin as CommonSpellPlugin,
    },
};

pub use instances::*;
pub use materials::*;

use crate::state::ClientState;

pub struct SpellPlugin;

pub const SPELLS_LABEL: &str = "spells";

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spawn_spells = SystemSet::on_update(ClientState::Arena)
            .label(SPELLS_LABEL)
            .after(CASTING_LABEL)
            .with_system(spawn_spells.system());
        app.init_resource::<SpellMaterials>()
            .add_plugin(CommonSpellPlugin::new(ClientState::Arena))
            .add_system_set(spawn_spells);
    }
}

fn spawn_spells(
    spell_materials: Res<SpellMaterials>,
    transform_query: Query<&Transform, With<CharacterMarker>>,
    mut spell_reader: EventReader<Spell>,

    mut commands: Commands,
) {
    for spell in spell_reader.iter() {
        match spell {
            Spell::Fireball { source, target } => {
                let common = CommonFireballBundle::new(*source, *target, 1.0);
                let transform = transform_query.get(source.0).expect("can't find caster");
                let bundle = FireballBundle::new(common, *transform, &spell_materials);

                commands.spawn_bundle(bundle);
            }
        }
    }
}
