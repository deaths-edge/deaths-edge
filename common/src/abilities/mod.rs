mod cast_type;
mod cooldowns;
mod cost;
mod instances;
mod instant;
mod lifecycle;
mod magic_type;
mod projectile;
mod requires_stationary;
mod requires_target;
mod spatial;

pub use cast_type::*;
pub use cooldowns::*;
pub use cost::*;
pub use instances::*;
pub use instant::*;
pub use lifecycle::*;
pub use magic_type::*;
pub use projectile::*;
pub use requires_stationary::*;
pub use requires_target::*;
pub use spatial::*;

use std::{fmt::Debug, hash::Hash};

use bevy::{prelude::*, utils::HashSet};

use crate::abilities::projectile::spawn_projectile;

#[derive(Default, Debug)]
pub struct AbilityMarker;

/// The character which the ability originates from.
#[derive(Debug)]
pub struct CharacterId(pub Entity);

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
        .insert(CharacterId(character_id))
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

        const INSTANCE_PREPARATION_LABEL: &str = "instance-preparation";
        let prepare_instances = SystemSet::on_update(self.state)
            .label(INSTANCE_PREPARATION_LABEL)
            .after(LIFECYCLE_LABEL)
            .with_system(adjoin_target.system());

        const PROJECTILE_PREPARATION_LABEL: &str = "instance-preparation";
        let prepare_projectiles = SystemSet::on_update(self.state)
            .label(PROJECTILE_PREPARATION_LABEL)
            .after(LIFECYCLE_LABEL)
            .with_system(adjoin_projectile_target.system());

        const CASTING_INSTANCES_LABEL: &str = "casting-instances";
        let casting_instances = SystemSet::on_update(self.state)
            .label(CASTING_INSTANCES_LABEL)
            .after(LIFECYCLE_LABEL)
            .with_system(motion_interrupt.system());

        const COMPLETE_INSTANCES_LABEL: &str = "complete-instances";
        let complete_instances = SystemSet::on_update(self.state)
            .label(COMPLETE_INSTANCES_LABEL)
            .after(LIFECYCLE_LABEL)
            .with_system(apply_health_cost.system())
            .with_system(apply_power_cost.system())
            .with_system(apply_damage.system())
            .with_system(apply_global_cooldown.system())
            .with_system(spawn_projectile.system());

        let inflight_projectiles =
            SystemSet::on_update(self.state).with_system(projectile_tracking.system());

        const LIFECYCLE_LABEL: &str = "lifecycle";
        let lifecycle = SystemSet::on_update(self.state)
            .label(LIFECYCLE_LABEL)
            .with_system(initialize_cast.system())
            .with_system(initialize_projectile.system())
            .with_system(complete_casting.system());

        let cleanup = SystemSet::on_update(self.state)
            .after(COMPLETE_INSTANCES_LABEL)
            .after(PROJECTILE_PREPARATION_LABEL)
            .with_system(remove_instance.system());

        app.add_system_set(lifecycle)
            .add_system_set(ability_checks)
            .add_system_set(prepare_instances)
            .add_system_set(prepare_projectiles)
            .add_system_set(casting_instances)
            .add_system_set(complete_instances)
            .add_system_set(inflight_projectiles)
            .add_system_set(cleanup);
    }
}
