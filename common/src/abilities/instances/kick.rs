use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        effects::{AtTarget, EffectMarker, Interrupt},
        lifecycle::InstantEffects,
        obstructions::{RequiresFov, RequiresTarget, UseObstructions},
        AbilityMarker,
    },
    dyn_command::DynEntityMutate,
};

#[derive(Debug, Clone, Bundle)]
pub struct PummelEffect {
    marker: EffectMarker,

    interrupt: AtTarget<Interrupt>,
}

#[derive(Debug, Bundle)]
pub struct Pummel {
    marker: AbilityMarker,
    instant_bundle: InstantEffects,

    requires_target: RequiresTarget,
    requires_fov: RequiresFov,
    obstructions: UseObstructions,
}

impl Pummel {
    pub fn new() -> Self {
        const INTERRUPT_LENGTH: Duration = Duration::from_secs(2);

        let effect = PummelEffect {
            marker: EffectMarker,

            interrupt: AtTarget(Interrupt(INTERRUPT_LENGTH)),
        };

        let pummel = Pummel {
            marker: AbilityMarker,
            instant_bundle: InstantEffects(DynEntityMutate::insert_bundle(effect)),

            requires_target: RequiresTarget::Enemy,
            requires_fov: RequiresFov,
            obstructions: UseObstructions::default(),
        };

        pummel
    }
}
