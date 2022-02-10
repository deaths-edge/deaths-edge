use bevy::prelude::*;
use heron::prelude::*;

use crate::{
    abilities::instances,
    character::{
        Abilities, CharacterBundle, CharacterIndex, CharacterMarker, Controls, Health,
        HealthRegenerate, LastCastInstant, Power, PowerRegenerate, SpeedMultiplier, Team,
    },
    physics::WorldLayer,
};

use super::{Class, ClassTrait};

#[derive(Bundle)]
pub struct Dummy {
    #[bundle]
    base_bundle: CharacterBundle,

    health_regenerate: HealthRegenerate,
    power_regenerate: PowerRegenerate,
}

pub const WIDTH: f32 = 30.0;
pub const HEIGHT: f32 = 30.0;

impl ClassTrait for Dummy {
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
            class: Class::Dummy,
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
                current: 500.0,
                total: 500.0,
            },
            health: Health {
                current: 500.0,
                total: 500.0,
            },
            abilities,

            last_cast_instant: LastCastInstant::default(),
        };

        let medea = Dummy {
            base_bundle,
            health_regenerate: HealthRegenerate(25.0),
            power_regenerate: PowerRegenerate(25.0),
        };
        commands.spawn_bundle(medea).id()
    }

    fn spawn_abilities(commands: &mut Commands) -> [Entity; 8] {
        let complain_id = commands
            .spawn()
            .insert_bundle(instances::Complain::new())
            .id();
        [
            complain_id,
            complain_id,
            complain_id,
            complain_id,
            complain_id,
            complain_id,
            complain_id,
            complain_id,
        ]
    }
}
