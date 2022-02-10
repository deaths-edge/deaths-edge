use bevy::prelude::*;

use super::HudState;

#[derive(Debug, Default, Component)]
pub struct UICameraMarker;

#[derive(Bundle, Default)]
pub struct UICamera {
    marker: UICameraMarker,
    #[bundle]
    ui_camera: UiCameraBundle,
}

pub fn setup_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UICamera::default());
}

pub struct UICameraPlugin;

impl Plugin for UICameraPlugin {
    fn build(&self, app: &mut App) {
        let setup = SystemSet::on_enter(HudState::Active).with_system(setup_ui_camera);
        app.add_system_set(setup);
    }
}
