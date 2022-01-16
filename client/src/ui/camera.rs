use bevy::prelude::*;

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
        app.add_startup_system(setup_ui_camera);
    }
}
