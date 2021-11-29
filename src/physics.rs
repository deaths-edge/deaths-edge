use std::ops::{Deref, DerefMut};

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    character::{CharacterIndex, CharacterMarker},
    environment::EnvironmentMarker,
    spells::{SpellImpactEvent, SpellMarker, SpellProjectileMarker, SpellTarget},
};

pub struct Velocity(Vec2);

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec2> for Velocity {
    fn from(val: Vec2) -> Self {
        Self(val)
    }
}

/// Requires [`SpellPlugin`](crate::spells::SpellPlugin) to be loaded.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let env_collisions = SystemSet::new()
            .label("env-collisions")
            .before("motion")
            .with_system(env_collisions.system());
        let spell_collisions = SystemSet::new()
            .label("spell-collisions")
            .before("motion")
            .with_system(spell_projectile_collisions.system());

        let motion = SystemSet::new()
            .label("motion")
            .after("env-collisions")
            .with_system(motion.system());
        app.add_system_set(env_collisions)
            .add_system_set(spell_collisions)
            .add_system_set(motion);
    }
}

pub fn env_collisions(
    environment_query: Query<(&Transform, &Sprite), With<EnvironmentMarker>>,
    mut player_query: Query<(
        &Transform,
        &Sprite,
        &mut Velocity,
        (With<CharacterMarker>, Without<EnvironmentMarker>),
    )>,
) {
    for (player_transform, player_sprite, mut velocity, _) in player_query.iter_mut() {
        for (env_transform, env_sprite) in &mut environment_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                env_transform.translation,
                env_sprite.size,
            );

            match collision {
                Some(Collision::Left) => {
                    velocity.0.x -= velocity.0.x.abs();
                    velocity.0.x /= 2.;
                }
                Some(Collision::Top) => {
                    velocity.0.y += velocity.0.y.abs();
                    velocity.0.y /= 2.;
                }
                Some(Collision::Right) => {
                    velocity.0.x += velocity.0.x.abs();
                    velocity.0.x /= 2.;
                }
                Some(Collision::Bottom) => {
                    velocity.0.y -= velocity.0.y.abs();
                    velocity.0.y /= 2.;
                }
                None => (),
            };
        }
    }
}

pub fn motion(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.) * time.delta_seconds();
    }
}

pub fn spell_projectile_collisions(
    mut spell_impact_events: EventWriter<SpellImpactEvent>,
    spell_query: Query<
        (Entity, &SpellMarker, &Transform, &Sprite, &SpellTarget),
        With<SpellProjectileMarker>,
    >,
    char_query: Query<
        (&CharacterIndex, &Transform, &Sprite),
        (With<CharacterMarker>, Without<SpellMarker>),
    >,

    commands: Commands,
) {
    for (spell_entity, spell_marker, spell_transform, spell_sprite, spell_target) in
        spell_query.iter()
    {
        let target_character_opt = char_query
            .iter()
            .find(|(index, _, _)| spell_target == *index);

        if let Some((_, target_transform, target_sprite)) = target_character_opt {
            let collision = collide(
                spell_transform.translation,
                spell_sprite.size,
                target_transform.translation,
                target_sprite.size,
            );

            match collision {
                Some(_) => {
                    let impact_event = SpellImpactEvent {
                        id: spell_entity,
                        spell_marker: *spell_marker,
                    };
                    spell_impact_events.send(impact_event);
                }
                None => (),
            };
        }
    }
}
