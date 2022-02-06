use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        effects::{AtSelf, AtTarget, Damage, EffectMarker, PowerBurn, TriggerGlobalCooldown},
        lifecycle::{CastMarker, InstantEffect, OnComplete, ProgressDuration, TotalDuration},
        magic_school::{Fire, Interruptable},
        obstructions::{
            CantWhileCasting, MaximumRange, OnGlobalCooldown, PowerCost, RequiresFov, RequiresLoS,
            RequiresStationary, RequiresTarget, UseObstructions,
        },
        AbilityMarker, Target,
    },
    dyn_command::EntityMutate,
};

use super::OnPress;

#[derive(Bundle, Clone)]
pub struct ScorchEffects {
    instant_marker: InstantEffect,
    effect_marker: EffectMarker,

    damage: AtTarget<Damage>,
    trigger_global_cooldown: AtSelf<TriggerGlobalCooldown>,

    power_cost: AtSelf<PowerBurn>,
}

#[derive(Debug, Clone, Bundle)]
pub struct ScorchCast {
    marker: CastMarker,

    fire_school: Fire,
    interruptable: Interruptable,

    requires_target: RequiresTarget,
    requires_stationary: RequiresStationary,
    requires_fov: RequiresFov,
    requires_los: RequiresLoS,
    max_range: MaximumRange,

    obstructions: UseObstructions,

    on_complete: OnComplete,

    progress_duration: ProgressDuration,
    total_duration: TotalDuration,
}

#[derive(Bundle)]
pub struct Scorch {
    marker: AbilityMarker,

    global_cooldown: OnGlobalCooldown,
    cant_while_casting: CantWhileCasting,

    fire_school: Fire,
    power_cost: PowerCost,

    requires_target: RequiresTarget,
    requires_stationary: RequiresStationary,
    requires_fov: RequiresFov,
    requires_los: RequiresLoS,
    max_range: MaximumRange,

    obstructions: UseObstructions,

    on_press: OnPress,
}

impl Scorch {
    pub fn new() -> Self {
        const DAMAGE: f32 = 30.0;
        const POWER_COST: f32 = 20.0;
        const CAST_DURATION: Duration = Duration::from_millis(750);
        const MAX_RANGE: f32 = 500.0;

        let scorch_effects = ScorchEffects {
            instant_marker: InstantEffect,
            effect_marker: EffectMarker,

            damage: AtTarget(Damage(DAMAGE)),

            trigger_global_cooldown: AtSelf(TriggerGlobalCooldown),
            power_cost: AtSelf(PowerBurn(POWER_COST)),
        };
        let effect_command = EntityMutate::new().insert_bundle(scorch_effects).arc();

        let scorch_cast = ScorchCast {
            marker: CastMarker,

            fire_school: Fire,
            interruptable: Interruptable,

            requires_target: RequiresTarget::Enemy,
            requires_stationary: RequiresStationary,
            requires_fov: RequiresFov,
            requires_los: RequiresLoS,
            max_range: MaximumRange(MAX_RANGE),
            obstructions: UseObstructions::default(),

            on_complete: OnComplete(effect_command),

            progress_duration: ProgressDuration::default(),
            total_duration: TotalDuration(CAST_DURATION),
        };

        let scorch_cast = EntityMutate::new()
            .insert_bundle(scorch_cast)
            .parent_source()
            .snapshot_clone::<Target>()
            .arc();

        Self {
            marker: AbilityMarker,

            global_cooldown: OnGlobalCooldown,
            cant_while_casting: CantWhileCasting,

            fire_school: Fire,
            power_cost: PowerCost(POWER_COST),

            requires_target: RequiresTarget::Enemy,
            requires_stationary: RequiresStationary,
            requires_fov: RequiresFov,
            requires_los: RequiresLoS,
            max_range: MaximumRange(MAX_RANGE),

            obstructions: UseObstructions::default(),

            on_press: OnPress(scorch_cast),
        }
    }
}
