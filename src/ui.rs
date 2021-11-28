use bevy::{app::Events, prelude::*, window::Windows};

use crate::character::{CharacterIndex, CharacterTarget, PlayerMarker};

use bevy::prelude::*;

pub struct PlayerCamera;

pub fn setup_camera(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(UiCameraBundle::default())
        .insert(PlayerCamera);
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<WorldMousePosition>()
            .add_system(select_highlight.system())
            .add_system(world_mouse.system());
    }
}

#[derive(Bundle)]
pub struct HealthBarBundle {
    #[bundle]
    bar: NodeBundle,
}

#[derive(Debug, Default)]
pub struct WorldMousePosition {
    pub position: Vec2,
}

pub fn window_to_local_position(
    windows: &Windows,
    camera_transform: &Transform,
    mouse_position: Vec2,
) -> Vec2 {
    // get the size of the window
    let primary_window = windows.get_primary().expect("no monitor");
    let size = Vec2::new(
        primary_window.width() as f32,
        primary_window.height() as f32,
    );

    let p = mouse_position - size / 2.0;

    let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
    Vec2::from(pos_wld)
}

pub fn world_mouse(
    windows: Res<Windows>,
    mut world_mouse_pos: ResMut<WorldMousePosition>,
    mouse_motion_events: Res<Events<CursorMoved>>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
    let mut mouse_pos_reader = mouse_motion_events.get_reader();
    if let Some(mouse_position) = mouse_pos_reader.iter(&mouse_motion_events).last() {
        let camera_transform = camera_query
            .single()
            .expect("there must be a player camera");

        let position =
            window_to_local_position(&windows, camera_transform, mouse_position.position);

        *world_mouse_pos = WorldMousePosition { position };
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Selected {
    Selected,
    Unselected,
}

impl Selected {
    pub fn is_selected(&self) -> bool {
        *self == Selected::Selected
    }
}

impl Default for Selected {
    fn default() -> Self {
        Self::Unselected
    }
}

pub fn select_highlight(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<
        (&mut Handle<ColorMaterial>, &Selected),
        (Changed<Selected>, With<CharacterIndex>),
    >,
) {
    for (mut material, selected) in query.iter_mut() {
        if selected.is_selected() {
            *material = materials.add(Color::OLIVE.into())
        } else {
            *material = materials.add(Color::MAROON.into())
        }
    }
}
