use bevy::prelude::*;

use crate::{
    character::{
        CharacterBundle, CharacterClass, CharacterIndex, CharacterMaterials, PlayerBundle,
    },
    ui::setup_nameplate,
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let player_spawn = spawn_player.system().chain(setup_nameplate.system());
        app.add_startup_system(player_spawn)
            .add_startup_system(spawn_char_1.system());
    }
}

pub fn spawn_player(
    time: Res<Time>,
    materials: Res<CharacterMaterials>,
    mut commands: Commands,
) -> Entity {
    let player_bundle = PlayerBundle::new(
        CharacterIndex::from(0),
        CharacterClass::Medea,
        &time,
        &materials,
    );
    let player_entity = commands.spawn_bundle(player_bundle).id();
    player_entity
}

pub fn spawn_char_1(time: Res<Time>, materials: Res<CharacterMaterials>, mut commands: Commands) {
    let character_bundle = CharacterBundle::new(
        CharacterIndex::from(1),
        CharacterClass::Heka,
        &time,
        &materials,
    );
    commands.spawn_bundle(character_bundle);
}
