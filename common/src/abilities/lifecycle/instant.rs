use bevy::prelude::*;

use crate::dyn_command::DynEntityMutate;

#[derive(Debug, Clone, Component)]
pub struct InstantEffects(pub DynEntityMutate);

#[derive(Debug, Clone, Component)]
pub struct InstantEffect;

/// Remove all the [`InstantEffect`]s after they are applied.
pub fn instant_effect_despawn(query: Query<Entity, With<InstantEffect>>, mut commands: Commands) {
    for instance_id in query.iter() {
        info!("cleaning up instant effect");
        commands.entity(instance_id).despawn();
    }
}
