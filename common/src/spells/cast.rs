use std::time::Duration;

use bevy::prelude::*;

use super::{SpellSource, SpellTarget};

#[derive(Debug)]
pub enum SpellCast {
    Fireball {
        source: SpellSource,
        target: SpellTarget,
    },
}

pub struct SpellTargeting<'a> {
    pub target: &'a SpellTarget,
    pub requires_fov: bool,
}

impl SpellCast {
    pub fn duration(&self) -> Duration {
        use SpellCast::*;

        match self {
            Fireball { .. } => Duration::from_secs(1),
        }
    }

    pub fn targeting(&self) -> Option<SpellTargeting<'_>> {
        match self {
            SpellCast::Fireball { target, .. } => Some(SpellTargeting {
                target,
                requires_fov: true,
            }),
        }
    }
}
