use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        effects::*,
        lifecycle::{InstantEffect, InstantEffects, StatusMarker},
        magic_school::Fire,
        obstructions::{
            CantWhileCasting, MaximumRange, OnCooldown, OnGlobalCooldown, PowerCost, RequiresFov,
            RequiresLoS, RequiresStationary, RequiresTarget, UseObstructions,
        },
        AbilityMarker,
    },
    character::LastCastInstant,
    dyn_command::DynEntityMutate,
};

const COOLDOWN: Duration = Duration::from_secs(8);

#[derive(Bundle, Clone)]
pub struct FireblastStatus {
    status_marker: StatusMarker,

    dot: AtTarget<Dot>,
}

#[derive(Bundle, Clone)]
pub struct FireblastEffects {
    instant_marker: InstantEffect,
    effect_marker: EffectMarker,

    damage: AtTarget<Damage>,
    cooldown: AtAbility<TriggerCooldown>,
    apply_status: AtTarget<ApplyStatus>,

    power_cost: AtSelf<PowerBurn>,
    trigger_global_cooldown: AtSelf<TriggerGlobalCooldown>,
}

#[derive(Bundle)]
pub struct Fireblast {
    marker: AbilityMarker,

    instant_bundle: InstantEffects,

    global_cooldown: OnGlobalCooldown,
    cooldown: OnCooldown,
    last_cast: LastCastInstant,

    power_cost: PowerCost,
    fire_school: Fire,

    requires_target: RequiresTarget,
    requires_stationary: RequiresStationary,
    requires_fov: RequiresFov,
    requires_los: RequiresLoS,
    max_range: MaximumRange,
    cant_while_casting: CantWhileCasting,

    obstructions: UseObstructions,
}

impl Fireblast {
    pub fn new() -> Self {
        const POWER_COST: f32 = 20.0;

        let fireblast_status = FireblastStatus {
            status_marker: StatusMarker,
            dot: AtTarget(Dot(10.0)),
        };

        let fireblast_effects = FireblastEffects {
            instant_marker: InstantEffect,
            effect_marker: EffectMarker,

            damage: AtTarget(Damage(25.0)),
            cooldown: AtAbility(TriggerCooldown(COOLDOWN)),
            apply_status: AtTarget(ApplyStatus(DynEntityMutate::insert_bundle(
                fireblast_status,
            ))),

            power_cost: AtSelf(PowerBurn(POWER_COST)),
            trigger_global_cooldown: AtSelf(TriggerGlobalCooldown),
        };
        let command = DynEntityMutate::insert_bundle(fireblast_effects);

        Self {
            marker: AbilityMarker,

            global_cooldown: OnGlobalCooldown,
            cooldown: OnCooldown(COOLDOWN),
            last_cast: LastCastInstant::default(),
            fire_school: Fire,

            power_cost: PowerCost(POWER_COST),

            requires_target: RequiresTarget::Enemy,
            requires_stationary: RequiresStationary,
            requires_fov: RequiresFov,
            requires_los: RequiresLoS,
            max_range: MaximumRange(500.0),
            cant_while_casting: CantWhileCasting,
            obstructions: UseObstructions::default(),

            instant_bundle: InstantEffects(command),
        }
    }
}
