use std::fmt::Debug;

use bevy::prelude::*;
use heron::prelude::*;
use serde::{Deserialize, Serialize};

use crate::physics::WorldLayer;

#[derive(Debug, Default, Component)]
pub struct EnvironmentMarker;

#[derive(Bundle)]
pub struct Environment {
    marker: EnvironmentMarker,

    // Physics
    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    collision_layer: CollisionLayers,

    #[bundle]
    sprite: SpriteBundle,
}

impl Environment {
    pub fn new(transform: Transform, size: Size) -> Self {
        Self {
            marker: EnvironmentMarker,

            rigid_body: RigidBody::Static,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec2::new(size.width / 2., size.height / 2.).extend(0.),
                border_radius: None,
            },
            collision_layer: CollisionLayers::none()
                .with_group(WorldLayer::Environment)
                .with_masks(&[WorldLayer::Character, WorldLayer::Spell]),

            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 1.0, 0.5),
                    custom_size: Some(Vec2::new(size.width, size.width)),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
pub enum Map {
    Duo,
}

impl Map {
    pub fn spawn_environment(&self, commands: &mut Commands) {
        match self {
            Self::Duo => {
                let pillar_a =
                    Environment::new(Transform::from_xyz(300., 300., 0.), Size::new(100., 100.));
                let pillar_b =
                    Environment::new(Transform::from_xyz(-300., -300., 0.), Size::new(100., 100.));

                commands.spawn_bundle(pillar_a);
                commands.spawn_bundle(pillar_b);
            }
        }
    }
}

pub struct EnvironmentPlugin<T> {
    state: T,
}

impl<T> EnvironmentPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

// impl<T> Plugin for EnvironmentPlugin<T>
// where
//     T: Sync + Send + Debug + Clone + Copy + Eq + Hash + 'static,
// {
//     fn build(&self, app: &mut AppBuilder) {
//         let spawn_environment =
//             SystemSet::on_enter(self.state).with_system(spawn_environment);

//         app.add_system(spawn_environment);
//     }
// }
