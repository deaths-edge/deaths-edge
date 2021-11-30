use bevy::prelude::*;

use crate::{
    character::{
        CharacterBundle, CharacterClass, CharacterIndex, CharacterMaterials, PlayerBundle,
    },
    ui::nameplate::setup_nameplate,
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let player_spawn = spawn_player.system().chain(setup_nameplate.system());
        let char_1_spawn = spawn_char_1.system().chain(setup_nameplate.system());
        app.add_startup_system(player_spawn)
            .add_startup_system(char_1_spawn);
    }
}

pub fn spawn_player(
    time: Res<Time>,
    materials: Res<CharacterMaterials>,
    mut commands: Commands,
) -> CharacterIndex {
    let index = CharacterIndex::from(0);
    let player_bundle = PlayerBundle::new(index, CharacterClass::Medea, &time, &materials);
    commands.spawn_bundle(player_bundle);
    index
}

pub fn spawn_char_1(
    time: Res<Time>,
    materials: Res<CharacterMaterials>,
    mut commands: Commands,
) -> CharacterIndex {
    let index = CharacterIndex::from(1);
    let character_bundle = CharacterBundle::new(index, CharacterClass::Heka, &time, &materials);
    commands.spawn_bundle(character_bundle);
    index
}
