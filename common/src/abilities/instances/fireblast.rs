use bevy::prelude::*;

use crate::abilities::{
    AbilityMarker, Damage, GlobalCooldown, InstantBundle, MaximumRange, PowerCost, RequiresFov,
    RequiresLoS, RequiresStationary, RequiresTarget,
};

#[derive(Bundle)]
pub struct FireballInstance {
    damage: Damage,
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
        Self {
            marker: AbilityMarker,

            global_cooldown: GlobalCooldown,

            power_cost: PowerCost(20.0),

            requires_target: RequiresTarget::Enemy,
            requires_stationary: RequiresStationary,
            requires_fov: RequiresFov,
            requires_los: RequiresLoS,
            max_range: MaximumRange(500.0),

            instant_bundle: InstantBundle(|| {
                Box::new(FireballInstance {
                    damage: Damage(25.0),
                })
            }),
        }
    }
}
