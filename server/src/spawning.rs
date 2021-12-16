use bevy::prelude::*;

use common::{network::server::SpawnCharacter, spawning::spawn_character_base};

use crate::{character::CharacterBundle, state::ServerState};

// TODO: Add waiting state
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spawner =
            SystemSet::on_update(ServerState::Running).with_system(spawn_characters.system());

        app.add_event::<SpawnCharacter>().add_system_set(spawner);
    }
}

pub fn spawn_characters(
    time: Res<Time>,
    mut spawn_events: EventReader<SpawnCharacter>,
    mut commands: Commands,
) {
    spawn_character_base(&time, &mut spawn_events, |common_bundle, spawn_event| {
        let position = spawn_event.position();
        let transform = Transform::from_xyz(position.x, position.y, 0.);

        let character_bundle = CharacterBundle::new(transform, common_bundle);
        commands.spawn_bundle(character_bundle);
    });
}
