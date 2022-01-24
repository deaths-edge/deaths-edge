use bevy::{prelude::*, utils::Instant};
use heron::prelude::*;

use crate::{
    abilities::instances,
    character::{
        Abilities, CastState, CharacterBundle, CharacterIndex, CharacterMarker, Controls, Health,
        LastCastInstant, OptionalTarget, Power, SpeedMultiplier, Team,
    },
    physics::WorldLayer,
};

use super::{Class, ClassTrait};

#[derive(Bundle)]
pub struct Mars {
    #[bundle]
    base_bundle: CharacterBundle,
}

pub const WIDTH: f32 = 25.0;
pub const HEIGHT: f32 = 25.0;

impl ClassTrait for Mars {
    fn spawn_character(
        index: CharacterIndex,
        team: Team,
        transform: Transform,
        abilities: Abilities,
        commands: &mut Commands,
    ) -> Entity {
        let base_bundle = CharacterBundle {
            index,
            marker: CharacterMarker,
            class: Class::Mars,
            team,
            transform,
            global_transform: Default::default(),
            speed_modifier: SpeedMultiplier(1.0),
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec2::new(WIDTH / 2.0, HEIGHT / 2.0).extend(0.0),
                border_radius: None,
            },
            collision_layers: CollisionLayers::none()
                .with_group(WorldLayer::Character)
                .with_mask(WorldLayer::Environment),
            velocity: Vec3::ZERO.into(),
            rotational_constraints: RotationConstraints::lock(),
            controls: Controls::default(),

            power: Power {
                current: 0.0,
                total: 200.0,
            },
            health: Health {
                current: 150.0,
                total: 150.0,
            },
            abilities,

            cast_state: CastState::default(),
            last_cast_instant: LastCastInstant::default(),
            target: OptionalTarget::default(),
        };

        let medea = Mars { base_bundle };
        commands.spawn_bundle(medea).id()
    }

    fn spawn_abilities(commands: &mut Commands) -> [Entity; 8] {
        let fireblast_id = commands
            .spawn()
            .insert_bundle(instances::Fireblast::new())
            .id();
        let scorch_id = commands
            .spawn()
            .insert_bundle(instances::Scorch::new())
            .id();
        [
            fireblast_id,
            scorch_id,
            scorch_id,
            scorch_id,
            scorch_id,
            scorch_id,
            scorch_id,
            scorch_id,
        ]
    }
}
