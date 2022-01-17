use bevy::prelude::*;

use crate::{
    abilities::{
        effects::{damage::Damage, power_burn::PowerBurn, AtSelf, AtTarget},
        AbilityMarker, GlobalCooldown, InstantBundle, MaximumRange, PowerCost, RequiresFov,
        RequiresLoS, RequiresStationary, RequiresTarget,
    },
    dyn_command::DynCommand,
};

#[derive(Bundle, Clone)]
pub struct FireballInstance {
    damage: AtTarget<Damage>,

    power_cost: AtSelf<PowerBurn>,
}

#[derive(Bundle)]
pub struct Fireblast {
    marker: AbilityMarker,

    global_cooldown: GlobalCooldown,

    power_cost: PowerCost,

    requires_target: RequiresTarget,
    requires_stationary: RequiresStationary,
    requires_fov: RequiresFov,
    requires_los: RequiresLoS,
    max_range: MaximumRange,

    instant_bundle: InstantBundle,
}

impl Fireblast {
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

            instant_bundle: InstantBundle(DynCommand::insert_bundle(FireballInstance {
                damage: AtTarget(Damage(25.0)),

                power_cost: AtSelf(PowerBurn(POWER_COST)),
            })),
        }
    }
}
