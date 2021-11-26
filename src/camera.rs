use bevy::prelude::*;

pub struct PlayerCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(UiCameraBundle::default())
        .insert(PlayerCamera);
}
