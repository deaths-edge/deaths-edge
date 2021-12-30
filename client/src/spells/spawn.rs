use bevy::prelude::*;

use common::{
    character::CharacterMarker,
    spells::{
        instances::{CommonFireballBundle, CommonSpearBundle},
        Spell,
    },
};

use super::{FireballBundle, SpearBundle, SpellMaterials};

pub fn spawn_spells(
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
            Spell::Spear { source, target } => {
                let common = CommonSpearBundle::new(*source, *target, 1.0);
                let transform = transform_query.get(source.0).expect("can't find caster");
                let bundle = SpearBundle::new(common, *transform, &spell_materials);

                commands.spawn_bundle(bundle);
            }
        }
    }
}
