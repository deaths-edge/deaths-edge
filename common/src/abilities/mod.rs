mod cast_type;
mod cooldown;
mod global_cooldown;
mod health_cost;
mod instances;
mod instant_damage;
mod lifecycle;
mod maximum_range;
mod power_cost;
mod projectile;
mod requires_fov;
mod requires_los;
mod requires_stationary;
mod requires_target;
mod spell_type;

pub use cast_type::*;
pub use cooldown::*;
pub use global_cooldown::*;
pub use health_cost::*;
pub use instances::*;
pub use instant_damage::*;
pub use lifecycle::*;
pub use maximum_range::*;
pub use power_cost::*;
pub use requires_fov::*;
pub use requires_los::*;
pub use requires_stationary::*;
pub use requires_target::*;
pub use spell_type::*;

use std::{fmt::Debug, hash::Hash};

use bevy::{prelude::*, utils::HashSet};

////
// These components will be in the platonic "ability"
// They are attached to each character via `AbilitySource`.

pub struct AbilityMarker;

/// The character which the ability originates from.
#[derive(Debug)]
pub struct AbilitySource(pub Entity);

/// An obstruction preventing a specific ability from being used.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Obstruction {
    InsufficientPower,
    OutOfRange,
    OutOfFOV,
    OutOfLoS,
    NoTarget,
    IncorrectTarget,
    Cooldown,
    GlobalCooldown,
    NonStationary,
    Locked,
}

pub fn spawn_class_abilities(character_id: Entity, commands: &mut Commands) {
    commands
        .spawn()
        .insert(AbilitySource(character_id))
        .insert_bundle(fireball::Fireball::new())
        .insert(UseObstructions::default());
}

#[derive(Debug, Default)]
pub struct UseObstructions(pub HashSet<Obstruction>);

pub struct AbilityPlugin<T> {
    state: T,
}

impl<T> AbilityPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for AbilityPlugin<T>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Hash + Eq,
{
    fn build(&self, app: &mut AppBuilder) {
        let ability_checks = SystemSet::on_update(self.state)
            // Geometric obstructions
            .with_system(check_required_target.system())
            .with_system(check_required_fov.system())
            .with_system(check_maximum_range.system())
            .with_system(check_required_stationary.system())
            // Resource obstructions
            .with_system(check_power_cost.system())
            // Cooldown obstructions
            .with_system(check_global_cooldown.system())
            .with_system(check_cooldown.system())
            // Check silence/lock
            // .with_system()
            ;

        const INSTANCE_PREPARATION: &str = "instance-preparation";

        let prepare_instances = SystemSet::on_update(self.state)
            .label(INSTANCE_PREPARATION)
            .with_system(adjoin_target.system());

        const ABILITY_APPLICATION: &str = "ability-application";

        let application = SystemSet::on_update(self.state)
            .label(ABILITY_APPLICATION)
            .with_system(apply_health_cost.system())
            .with_system(apply_power_cost.system())
            .with_system(apply_damage.system())
            .with_system(apply_global_cooldown.system());

        let lifecycle = SystemSet::on_update(self.state)
            .with_system(initialize_cast.system())
            .with_system(complete_casting.system())
            .with_system(remove_instance.system());

        app.add_system_set(ability_checks)
            .add_system_set(prepare_instances)
            .add_system_set(application);
    }
}
