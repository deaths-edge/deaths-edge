use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        effects::{damage::Damage, power_burn::PowerBurn, AtSelf, AtTarget},
        AbilityMarker, CastBundle, CastDuration, GlobalCooldown, MaximumRange, PowerCost,
        RequiresFov, RequiresLoS, RequiresStationary, RequiresTarget,
    },
    dyn_command::DynCommand,
};

#[derive(Bundle, Clone)]
pub struct ScorchInstance {
    damage: AtTarget<Damage>,

    power_cost: AtSelf<PowerBurn>,
}

#[derive(Debug, Clone, Bundle)]
pub struct ScorchCast {
    duration: CastDuration,
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

    cast_bundle: CastBundle,
}

impl Scorch {
    pub fn new() -> Self {
        const POWER_COST: f32 = 20.0;
        Self {
            marker: AbilityMarker,

            global_cooldown: GlobalCooldown,

            power_cost: PowerCost(POWER_COST),

            requires_target: RequiresTarget::Enemy,
            requires_stationary: RequiresStationary,
            requires_fov: RequiresFov,
            requires_los: RequiresLoS,
            max_range: MaximumRange(500.0),

            cast_bundle: CastBundle(DynCommand::insert_bundle(ScorchCast {
                duration: CastDuration(Duration::from_secs(1)),
            })),
        }
    }
}
