pub mod checks;
pub mod effects;
pub mod instances;
mod lifecycle;
mod magic_school;
mod target;

pub use checks::*;
pub use lifecycle::*;
pub use magic_school::*;
pub use target::*;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use effects::{cleanup, damage::Damage, EffectPlugin};

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
        .insert_bundle(instances::Fireblast::new())
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
    EffectApplication,
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

        let effects = EffectPlugin::new(self.state, AbilityLabels::EffectApplication);

        app.add_system_set(ability_checks).add_plugin(effects);
    }
}
