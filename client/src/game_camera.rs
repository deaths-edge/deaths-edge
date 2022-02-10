use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CameraState {
    Active,
    Inactive,
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        let setup_camera = SystemSet::on_enter(CameraState::Active).with_system(setup_camera);

        // TODO: Deactivate camera
        app.add_state(CameraState::Inactive)
            .add_system_set(setup_camera);
    }
}
