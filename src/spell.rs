use std::time::Duration;

use bevy::prelude::*;

use crate::{
    character::{CharacterIndex, CharacterTarget},
    physics::Velocity,
};

pub struct SpellMarker;

pub struct SpellTracking;

#[derive(Clone, Copy)]
pub struct SpellTarget(CharacterIndex);

impl From<CharacterIndex> for SpellTarget {
    fn from(value: CharacterIndex) -> Self {
        Self(value)
    }
}

impl PartialEq<CharacterIndex> for SpellTarget {
    fn eq(&self, other: &CharacterIndex) -> bool {
        self.0 == *other
    }
}

pub struct SpellIndex(usize);

impl From<usize> for SpellIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub enum Spell {
    Fireball,
}

#[derive(Bundle)]
pub struct FireballBundle {
    marker: SpellMarker,
    #[bundle]
    sprite: SpriteBundle,
    target: SpellTarget,
    tracking: SpellTracking,
    velocity: Velocity,
}

impl Spell {
    pub fn duration(&self) -> Duration {
        use Spell::*;

        match self {
            Fireball => Duration::from_secs(2),
        }
    }

    pub fn spawn_bundle(
        &self,
        parent: Entity,
        transform: &Transform,
        target: &CharacterTarget,
        commands: &mut Commands,
        materials: &mut Assets<ColorMaterial>,
    ) {
        use Spell::*;

        match self {
            Fireball => {
                const FIREBALL_SPEED: f32 = 300.;

                let entity_commands = commands.spawn_bundle(FireballBundle {
                    marker: SpellMarker,
                    sprite: SpriteBundle {
                        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
                        sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                        transform: *transform,
                        ..Default::default()
                    },
                    target: SpellTarget::from(target.target.expect("fucked")),
                    tracking: SpellTracking,
                    velocity: Velocity::from(Vec2::new(FIREBALL_SPEED, 0.)),
                });

                let child_id = entity_commands.id();

                // commands.entity(parent).push_children(&[child_id]);
            }
        }
    }
}
