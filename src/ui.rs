use bevy::{app::Events, prelude::*, window::Windows};

use crate::camera::PlayerCamera;

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
