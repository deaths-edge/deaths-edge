use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        effects::*,
        lifecycle::{Complete, ProgressDuration, StatusMarker, TotalDuration},
        magic_school::Fire,
        obstructions::{
            CantWhileCasting, MaximumRange, OnCooldown, OnGlobalCooldown, PowerCost, RequiresFov,
            RequiresLoS, RequiresStationary, RequiresTarget, UseObstructions,
        },
        AbilityMarker, Source, Target,
    },
    character::LastCastInstant,
    dyn_command::EntityMutate,
};

use super::OnPress;

const COOLDOWN: Duration = Duration::from_secs(8);

#[derive(Bundle, Clone)]
pub struct FireblastStatus {
    status_marker: StatusMarker,
    effect_marker: EffectMarker,

    dot: AtTarget<Dot>,

    progress_duration: ProgressDuration,
    total_duration: TotalDuration,
}

#[derive(Bundle, Clone)]
pub struct FireblastEffects {
    effect_marker: EffectMarker,

    damage: AtTarget<Damage>,
    apply_status: AtTarget<SpawnEntity>,
    power_cost: AtSelf<PowerBurn>,
    trigger_cooldown: AtAbility<TriggerCooldown>,
    trigger_global_cooldown: AtSelf<TriggerGlobalCooldown>,

    complete: Complete,
}

#[derive(Bundle)]
pub struct Fireblast {
    marker: AbilityMarker,

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

    on_press: OnPress,

    obstructions: UseObstructions,
}

impl Fireblast {
    pub fn new() -> Self {
        const POWER_COST: f32 = 20.0;
        const DOT_DURATION: Duration = Duration::from_secs(2);

        let fireblast_status = FireblastStatus {
            status_marker: StatusMarker,
            effect_marker: EffectMarker,
            dot: AtTarget(Dot(10.0)),

            progress_duration: ProgressDuration::default(),
            total_duration: TotalDuration(DOT_DURATION),
        };

        let fireblast_effects = FireblastEffects {
            effect_marker: EffectMarker,

            damage: AtTarget(Damage(25.0)),
            apply_status: AtTarget(SpawnEntity(
                EntityMutate::new()
                    .insert_bundle(fireblast_status)
                    .snapshot_clone::<Source>()
                    .snapshot_clone::<Target>()
                    .arc(),
            )),
            power_cost: AtSelf(PowerBurn(POWER_COST)),
            trigger_global_cooldown: AtSelf(TriggerGlobalCooldown),
            trigger_cooldown: AtAbility(TriggerCooldown(COOLDOWN)),

            complete: Complete,
        };
        let entity_mutate = EntityMutate::new()
            .insert_bundle(fireblast_effects)
            .parent_source()
            .snapshot_clone::<Target>()
            .arc();

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

            on_press: OnPress(entity_mutate),
        }
    }
}
