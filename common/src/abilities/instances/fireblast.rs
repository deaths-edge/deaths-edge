use bevy::prelude::*;

use crate::{
    abilities::{
        effects::{AtSelf, AtTarget, Damage, EffectMarker, PowerBurn, TriggerGlobalCooldown},
        lifecycle::InstantBundle,
        obstructions::{
            GlobalCooldown, MaximumRange, PowerCost, RequiresFov, RequiresLoS, RequiresStationary,
            RequiresTarget, UseObstructions,
        },
        AbilityMarker,
    },
    dyn_command::DynCommand,
};

#[derive(Bundle, Clone)]
pub struct FireblastEffects {
    marker: EffectMarker,

    damage: AtTarget<Damage>,

    power_cost: AtSelf<PowerBurn>,
    trigger_global_cooldown: AtSelf<TriggerGlobalCooldown>,
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
    obstructions: UseObstructions,

    instant_bundle: InstantBundle,
}

impl Fireblast {
    pub fn new() -> Self {
        const POWER_COST: f32 = 20.0;

        let fireblast_effects = FireblastEffects {
            marker: EffectMarker,

            damage: AtTarget(Damage(25.0)),

            power_cost: AtSelf(PowerBurn(POWER_COST)),
            trigger_global_cooldown: AtSelf(TriggerGlobalCooldown),
        };
        let command = DynCommand::insert_bundle(fireblast_effects);

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

            instant_bundle: InstantBundle(command),
        }
    }
}
