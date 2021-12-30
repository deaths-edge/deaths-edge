use bevy::prelude::*;

use crate::effects::{EffectMarker, EffectTarget, InterruptEffect};

#[derive(Bundle)]
pub struct MovementInterruptBundle {
    effect_marker: EffectMarker,
    interrupt: InterruptEffect,
    target: EffectTarget,
}

impl MovementInterruptBundle {
    pub fn new(target: EffectTarget) -> Self {
        Self {
            effect_marker: EffectMarker,
            interrupt: InterruptEffect::default(),
            target,
        }
    }
}
