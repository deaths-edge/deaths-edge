use bevy::prelude::*;
use heron::prelude::*;

use crate::{
    abilities::instances,
    character::{
        Abilities, CharacterBundle, CharacterIndex, CharacterMarker, Controls, Health,
        LastCastInstant, Power, PowerRegenerate, SpeedMultiplier, Team,
    },
    physics::WorldLayer,
};

use super::{Class, ClassTrait};

#[derive(Bundle)]
pub struct Medea {
    #[bundle]
    base_bundle: CharacterBundle,

    power_regenerate: PowerRegenerate,
}

pub const WIDTH: f32 = 20.0;
pub const HEIGHT: f32 = 20.0;

impl ClassTrait for Medea {
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
            class: Class::Medea,
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

            last_cast_instant: LastCastInstant::default(),
        };

        let medea = Medea {
            base_bundle,
            power_regenerate: PowerRegenerate(20.0),
        };

        commands.spawn_bundle(medea).id()
    }

    fn spawn_abilities(commands: &mut Commands) -> [Entity; 8] {
        let blink_id = commands.spawn_bundle(instances::Blink::new()).id();
        let fireblast_id = commands.spawn_bundle(instances::Fireblast::new()).id();
        let scorch_id = commands.spawn_bundle(instances::Scorch::new()).id();

        [
            blink_id,
            fireblast_id,
            scorch_id,
            scorch_id,
            scorch_id,
            scorch_id,
            scorch_id,
            scorch_id,
        ]
    }
}
