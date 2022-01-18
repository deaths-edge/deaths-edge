use bevy::{ecs::system::EntityCommands, prelude::*, utils::Instant};
use heron::prelude::*;

use crate::{
    abilities::{instances, AbilityId, UseObstructions},
    character::{
        Abilities, CastState, CharacterBundle, CharacterIndex, CharacterMarker, Controls, Health,
        LastCastInstant, OptionalTarget, Power, SpeedMultiplier, Team,
    },
    physics::WorldLayer,
};

use super::Class;

#[derive(Bundle)]
pub struct Mars {
    #[bundle]
    base_bundle: CharacterBundle,
}

pub const WIDTH: f32 = 25.0;
pub const HEIGHT: f32 = 25.0;

impl Mars {
    pub fn spawn_abilities(commands: &mut Commands) -> Abilities {
        // TODO
        let fireblast_id = commands
            .spawn()
            .insert_bundle(instances::Fireblast::new())
            .insert(UseObstructions::default())
            .id();
        let scorch_id = commands
            .spawn()
            .insert_bundle(instances::Scorch::new())
            .insert(UseObstructions::default())
            .id();

        Abilities(
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
            .map(AbilityId),
        )
    }

    pub fn spawn<'a, 'w, 's>(
        index: CharacterIndex,
        team: Team,
        transform: Transform,
        last_cast_instant: Instant,
        commands: &'a mut Commands<'w, 's>,
    ) -> EntityCommands<'w, 's, 'a> {
        let abilities = Self::spawn_abilities(commands);

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
                current: 200.0,
                total: 200.0,
            },
            health: Health {
                current: 150.0,
                total: 150.0,
            },
            abilities,

            cast_state: CastState::default(),
            last_cast_instant: LastCastInstant(last_cast_instant),
            target: OptionalTarget::default(),
        };

        let medea = Mars { base_bundle };
        commands.spawn_bundle(medea)
    }
}
