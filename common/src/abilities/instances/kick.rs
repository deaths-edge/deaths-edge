use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        effects::{AtTarget, EffectMarker, Interrupt},
        lifecycle::Complete,
        obstructions::{RequiresFov, RequiresTarget, UseObstructions},
        AbilityMarker, Target,
    },
    dyn_command::EntityMutate,
};

use super::OnPress;

#[derive(Debug, Clone, Bundle)]
pub struct PummelEffect {
    marker: EffectMarker,

    interrupt: AtTarget<Interrupt>,

    complete: Complete,
}

#[derive(Debug, Bundle)]
pub struct Pummel {
    marker: AbilityMarker,

    requires_target: RequiresTarget,
    requires_fov: RequiresFov,
    obstructions: UseObstructions,

    on_press: OnPress,
}

impl Pummel {
    pub fn new() -> Self {
        const INTERRUPT_LENGTH: Duration = Duration::from_secs(2);

        let effect = PummelEffect {
            marker: EffectMarker,

            interrupt: AtTarget(Interrupt(INTERRUPT_LENGTH)),

            complete: Complete,
        };

        let pummel = Pummel {
            marker: AbilityMarker,
            requires_target: RequiresTarget::Enemy,
            requires_fov: RequiresFov,
            obstructions: UseObstructions::default(),

            on_press: OnPress(
                EntityMutate::new()
                    .insert_bundle(effect)
                    .parent_source()
                    .snapshot_clone::<Target>()
                    .arc(),
            ),
        };

        pummel
    }
}
