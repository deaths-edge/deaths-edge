use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct StatusMarker;

#[derive(Debug, Component)]
pub struct StatusDuration(pub Duration);

#[derive(Debug, Component)]
pub struct Dispelled;

pub fn cleanup_dispelled(
    time: Res<Time>,
    query: Query<Entity, With<StatusMarker>>,
    mut commands: Commands,
) {
    let now = time.last_update().expect("failed to find last update");
    for status_id in query.iter() {
        commands.entity(status_id).despawn();
    }
}
