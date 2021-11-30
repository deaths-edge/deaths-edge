pub mod mouse;
pub mod nameplate;
pub mod selected;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct PlayerCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(UiCameraBundle::default())
        .insert(PlayerCamera);
}

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(mouse::WorldMousePlugin)
            .add(nameplate::NameplatePlugin);
    }
}
