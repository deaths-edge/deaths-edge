use bevy::window::WindowDescriptor;

pub fn window_description() -> WindowDescriptor {
    WindowDescriptor {
        title: "Death's Edge".to_string(),
        width: 800.,
        height: 600.,
        ..Default::default()
    }
}
