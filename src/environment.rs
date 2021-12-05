use bevy::prelude::*;
use heron::prelude::*;

pub struct EnvironmentMarker;

#[derive(Bundle)]
pub struct Environment {
    marker: EnvironmentMarker,

    // Physics
    rigid_body: RigidBody,
    collision_shape: CollisionShape,

    #[bundle]
    sprite: SpriteBundle,
}

impl Environment {
    pub fn new(transform: Transform, size: Size, materials: &mut Assets<ColorMaterial>) -> Self {
        Self {
            marker: EnvironmentMarker,

            rigid_body: RigidBody::Static,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec2::new(size.width / 2., size.height / 2.).extend(0.),
                border_radius: None,
            },

            sprite: SpriteBundle {
                material: materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
                sprite: Sprite::new(Vec2::new(size.width, size.width)),
                transform,
                ..Default::default()
            },
        }
    }
}

pub fn spawn_environment(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let environment_a = Environment::new(
        Transform::from_xyz(300., 300., 0.),
        Size::new(100., 100.),
        &mut materials,
    );
    let environment_b = Environment::new(
        Transform::from_xyz(-300., -300., 0.),
        Size::new(100., 100.),
        &mut materials,
    );

    commands.spawn_bundle(environment_a);
    commands.spawn_bundle(environment_b);
}
