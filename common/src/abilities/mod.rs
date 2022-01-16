mod cast;
mod checks;
mod damage;
mod instances;
mod instant;
mod magic_school;
mod target;

pub use cast::*;
pub use checks::*;
pub use damage::*;
pub use instances::*;
pub use instant::*;
pub use magic_school::*;
pub use target::*;

use std::{fmt::Debug, hash::Hash};

use bevy::{prelude::*, utils::HashSet};

#[derive(Default, Debug, Component)]
pub struct AbilityMarker;

/// The character which the ability originates from.
#[derive(Debug, Component)]
pub struct CharacterId(pub Entity);

pub fn spawn_class_abilities(character_id: Entity, commands: &mut Commands) {
    // use Class::*;
    // match class {
    //     Class
    // }
    commands
        .spawn()
        .insert(CharacterId(character_id))
        .insert_bundle(fireblast::Fireblast::new())
        .insert(UseObstructions::default());
}

pub struct AbilityPlugin<T> {
    state: T,
}

impl<T> AbilityPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbilityLabels {
    Checks,
    Cleanup,
}

impl SystemLabel for AbilityLabels {
    fn dyn_clone(&self) -> Box<dyn SystemLabel> {
        Box::new(self.clone())
    }
}

impl<T> Plugin for AbilityPlugin<T>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Hash + Eq,
{
    fn build(&self, app: &mut App) {
        let ability_checks = SystemSet::on_update(self.state)
            .label(AbilityLabels::Checks)
            // Geometric obstructions
            .with_system(check_required_target)
            .with_system(check_required_fov)
            .with_system(check_maximum_range)
            .with_system(check_required_stationary)
            // Resource obstructions
            .with_system(check_power_cost)
            // Cooldown obstructions
            .with_system(check_global_cooldown)
            .with_system(check_cooldown)
            // Check silence/lock
            // .with_system()
            ;

        let cleanup = SystemSet::on_update(self.state)
            .label(AbilityLabels::Cleanup)
            .with_system(cleanup);

        app.add_system_set(ability_checks).add_system_set(cleanup);
    }
}
