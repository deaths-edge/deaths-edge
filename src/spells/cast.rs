use std::time::Duration;

use bevy::prelude::*;

use super::{SpellSource, SpellTarget};
use crate::spells::instances::FireballBundle;

#[derive(Debug)]
pub enum SpellCast {
    Fireball {
        source: SpellSource,
        target: SpellTarget,
    },
}

impl SpellCast {
    pub fn duration(&self) -> Duration {
        use SpellCast::*;

        match self {
            Fireball { .. } => Duration::from_secs(1),
        }
    }

    pub fn spawn_bundle(
        &self,
        parent: Entity,
        transform: &Transform,
        commands: &mut Commands,
        materials: &mut Assets<ColorMaterial>,
    ) {
        use SpellCast::*;

        match self {
            Fireball { source, target } => {
                commands.spawn_bundle(FireballBundle::new(
                    *transform, *source, *target, 1.0, materials,
                ));
            }
        }
    }
}
