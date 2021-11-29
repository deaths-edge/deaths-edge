mod nameplate;
mod selected;

use bevy::{app::Events, prelude::*, window::Windows};

pub use nameplate::*;
pub use selected::*;

pub struct PlayerCamera;

pub fn setup_camera(mut commands: Commands) {
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
            .init_resource::<NameplateMaterials>()
            .add_system(select_highlight.system())
            .add_system(world_mouse.system());
    }
}

#[derive(Debug)]
pub struct HealthBarMarker;

#[derive(Debug, Bundle)]
pub struct HealthBarBundle {
    marker: HealthBarMarker,
    #[bundle]
    node: NodeBundle,
}

impl HealthBarBundle {
    pub fn new(nameplate_materials: &NameplateMaterials) -> Self {
        Self {
            marker: HealthBarMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(50.)),
                    ..Default::default()
                },
                material: nameplate_materials.primary_health_bar.clone(),
                ..Default::default()
            },
        }
    }
}

pub fn setup_nameplate(
    player_entity: In<Entity>,

    nameplate_materials: Res<NameplateMaterials>,

    mut commands: Commands,
) {
    let nameplate_bundle = NameplateBundle::new(&nameplate_materials);
    commands
        .spawn_bundle(nameplate_bundle)
        .with_children(|commands| {
            let health_bar_bundle = HealthBarBundle::new(&nameplate_materials);
            let power_bar_bundle = PowerBarBundle::new(&nameplate_materials);

            commands.spawn_bundle(health_bar_bundle);
            commands.spawn_bundle(power_bar_bundle);
        });
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
