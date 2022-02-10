use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        effects::{
            AtAbility, AtSelf, Displace, EffectMarker, TriggerCooldown, TriggerGlobalCooldown,
        },
        lifecycle::Complete,
        obstructions::{CantWhileCasting, OnCooldown, OnGlobalCooldown, UseObstructions},
        AbilityMarker,
    },
    character::LastCastInstant,
    dyn_command::EntityMutate,
};

use super::OnPress;

#[derive(Debug, Clone, Bundle)]
pub struct BlinkEffect {
    marker: EffectMarker,

    interrupt: AtSelf<Displace>,
    trigger_cooldown: AtAbility<TriggerCooldown>,
    trigger_global_cooldown: AtSelf<TriggerGlobalCooldown>,

    complete: Complete,
}

#[derive(Debug, Bundle)]
pub struct Blink {
    marker: AbilityMarker,

    cooldown: OnCooldown,
    last_cast: LastCastInstant,

    global_cooldown: OnGlobalCooldown,
    cant_while_casting: CantWhileCasting,

    obstructions: UseObstructions,

    on_press: OnPress,
}

impl Blink {
    pub const BLINK_DISTANCE: f32 = 250.0;
    pub const COOLDOWN: Duration = Duration::from_secs(12);

    pub fn new() -> Self {
        let effect = BlinkEffect {
            marker: EffectMarker,

            interrupt: AtSelf(Displace(Vec2::new(0.0, Self::BLINK_DISTANCE))),
            trigger_cooldown: AtAbility(TriggerCooldown(Self::COOLDOWN)),
            trigger_global_cooldown: AtSelf(TriggerGlobalCooldown),

            complete: Complete,
        };

        let pummel = Blink {
            marker: AbilityMarker,

            cooldown: OnCooldown(Self::COOLDOWN),
            last_cast: LastCastInstant::default(),

            global_cooldown: OnGlobalCooldown,
            cant_while_casting: CantWhileCasting,
            obstructions: UseObstructions::default(),

            on_press: OnPress(
                EntityMutate::new()
                    .insert_bundle(effect)
                    .parent_source()
                    .arc(),
            ),
        };

        pummel
    }
}
