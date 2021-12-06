use bevy::prelude::*;

use crate::{
    character::{
        CharacterBundle, CharacterClass, CharacterIndex, CharacterMaterials, PlayerBundle,
    },
    state::AppState,
    ui::nameplate::setup_nameplate,
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        let char_spawn = SystemSet::on_enter(AppState::Arena)
            .with_system(spawn_player.system().chain(setup_nameplate.system()))
            .with_system(spawn_char_1.system().chain(setup_nameplate.system()));

        app.add_system_set(char_spawn);
    }
}

pub fn spawn_player(
    time: Res<Time>,
    materials: Res<CharacterMaterials>,
    mut commands: Commands,
) -> Entity {
    let index = CharacterIndex::from(0);
    let player_bundle = PlayerBundle::new(
        index,
        CharacterClass::Medea,
        Transform::from_xyz(50., 50., 0.),
        &time,
        &materials,
    );
    commands.spawn_bundle(player_bundle).id()
}

pub fn spawn_char_1(
    time: Res<Time>,
    materials: Res<CharacterMaterials>,
    mut commands: Commands,
) -> Entity {
    let index = CharacterIndex::from(1);
    let character_bundle = CharacterBundle::new(
        index,
        CharacterClass::Heka,
        Transform::from_xyz(-50., -50., 0.),
        &time,
        &materials,
    );
    commands.spawn_bundle(character_bundle).id()
}
