use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct InstantEffect;

/// Remove all the [`InstantEffect`]s after they are applied.
pub fn instant_effect_despawn(query: Query<Entity, With<InstantEffect>>, mut commands: Commands) {
    for instance_id in query.iter() {
        info!("cleaning up instant effect");
        commands.entity(instance_id).despawn();
    }
}

pub struct InstantPlugin<T, L> {
    pub state: T,
    pub label: L,
}

impl<T, L> Plugin for InstantPlugin<T, L>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,
{
    fn build(&self, app: &mut App) {
        let set = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .with_system(instant_effect_despawn);
        app.add_system_set(set);
    }
}
