use std::ops::{Deref, DerefMut};

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{character::CharacterIndex, environment::Environment};

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

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let collisions = SystemSet::new()
            .label("collisions")
            .before("kinetics")
            .with_system(collisions.system());
        let kinetics = SystemSet::new()
            .label("kinetics")
            .after("collisions")
            .with_system(kinetics.system());
        app.add_system_set(collisions).add_system_set(kinetics);
    }
}

pub fn collisions(
    environment_query: Query<(&Environment, &Transform, &Sprite)>,
    mut player_query: Query<(
        &CharacterIndex,
        &mut Transform,
        &Sprite,
        &mut Velocity,
        Without<Environment>,
    )>,
) {
    for (_, player_transform, player_sprite, mut velocity, _) in player_query.iter_mut() {
        for (_, env_transform, env_sprite) in &mut environment_query.iter() {
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

pub fn kinetics(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.) * time.delta_seconds();
    }
}
