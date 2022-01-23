use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        effects::{AtSelf, AtTarget, Damage, EffectMarker, PowerBurn, TriggerGlobalCooldown},
        lifecycle::{CastBundle, CastDuration, CastMarker, InstantBundle},
        obstructions::{
            GlobalCooldown, MaximumRange, PowerCost, RequiresFov, RequiresLoS, RequiresStationary,
            RequiresTarget, UseObstructions,
        },
        AbilityMarker,
    },
    dyn_command::DynCommand,
};

#[derive(Bundle, Clone)]
pub struct ScorchEffects {
    marker: EffectMarker,

    damage: AtTarget<Damage>,

    power_cost: AtSelf<PowerBurn>,
    trigger_global_cooldown: AtSelf<TriggerGlobalCooldown>,
}

#[derive(Debug, Clone, Bundle)]
pub struct ScorchCast {
    marker: CastMarker,
    duration: CastDuration,

    instant_bundle: InstantBundle,

    requires_stationary: RequiresStationary,
}

#[derive(Bundle)]
pub struct Scorch {
    marker: AbilityMarker,

    global_cooldown: GlobalCooldown,

    power_cost: PowerCost,

    requires_target: RequiresTarget,
    requires_stationary: RequiresStationary,
    requires_fov: RequiresFov,
    requires_los: RequiresLoS,
    max_range: MaximumRange,
    obstructions: UseObstructions,

    cast_bundle: CastBundle,
}

impl Scorch {
    pub fn new() -> Self {
        const DAMAGE: f32 = 30.0;
        const POWER_COST: f32 = 20.0;

        let scorch_effects = ScorchEffects {
            marker: EffectMarker,

            damage: AtTarget(Damage(DAMAGE)),
            power_cost: AtSelf(PowerBurn(POWER_COST)),

            trigger_global_cooldown: AtSelf(TriggerGlobalCooldown),
        };
        let effect_command = DynCommand::insert_bundle(scorch_effects);

        let scorch_cast = ScorchCast {
            marker: CastMarker,
            duration: CastDuration(Duration::from_secs(1)),
            instant_bundle: InstantBundle(effect_command),

            requires_stationary: RequiresStationary,
        };
        let scorch_cast_command = DynCommand::insert_bundle(scorch_cast);

        Self {
            marker: AbilityMarker,

            global_cooldown: GlobalCooldown,

            power_cost: PowerCost(POWER_COST),

            requires_target: RequiresTarget::Enemy,
            requires_stationary: RequiresStationary,
            requires_fov: RequiresFov,
            requires_los: RequiresLoS,
            max_range: MaximumRange(500.0),
            obstructions: UseObstructions::default(),

            cast_bundle: CastBundle(scorch_cast_command),
        }
    }
}
