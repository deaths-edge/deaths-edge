use bevy::prelude::*;

#[derive(Component)]
pub struct InstantEffectsMarker;

#[derive(Component)]
pub struct InstantBundle(pub fn() -> Box<dyn ApplicableBundle>);

pub fn cleanup(query: Query<Entity, With<InstantEffectsMarker>>, mut commands: Commands) {
    for instance_id in query.iter() {
        commands.entity(instance_id).despawn();
    }
}
