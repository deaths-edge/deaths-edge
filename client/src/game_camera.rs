use bevy::prelude::*;

use crate::state::ClientState;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        let setup_camera = SystemSet::on_enter(ClientState::Arena).with_system(setup_camera);
        app.add_system_set(setup_camera);
    }
}
