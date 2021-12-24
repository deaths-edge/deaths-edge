mod impact;
mod index;
pub mod instances;
mod marker;
mod projectiles;
mod source;
mod target;
mod utilities;

use std::{fmt::Debug, hash::Hash, time::Duration};

use bevy::prelude::*;
use heron::{prelude::*, rapier_plugin::PhysicsWorld};

pub use impact::*;
pub use marker::*;
pub use projectiles::*;
pub use source::*;
pub use target::*;
pub use utilities::*;

use crate::character::{CharacterMarker, LastCastInstant};

/// Enumeration of all spells.
#[derive(Debug)]
pub enum Spell {
    Fireball {
        source: SpellSource,
        target: SpellTarget,
    },
    Spear {
        source: SpellSource,
        target: SpellTarget,
    },
}

pub struct SpellTargeting<'a> {
    pub target: &'a SpellTarget,
    pub requires_fov: bool,
}

impl Spell {
    pub fn duration(&self) -> Option<Duration> {
        use Spell::*;

        match self {
            Fireball { .. } => Some(Duration::from_secs(1)),
            Spear { .. } => None,
        }
    }

    pub fn targeting(&self) -> Option<SpellTargeting<'_>> {
        match self {
            Spell::Fireball { target, .. } => Some(SpellTargeting {
                target,
                requires_fov: true,
            }),
            Spell::Spear { target, .. } => Some(SpellTargeting {
                target,
                requires_fov: true,
            }),
        }
    }
}

pub struct SpellPlugin<State, Trigger> {
    state: State,
    trigger: Trigger,
}

impl<State, Trigger> SpellPlugin<State, Trigger> {
    pub fn new(state: State, trigger: Trigger) -> Self {
        Self { state, trigger }
    }
}

pub const SPELLS_LABEL: &str = "spells";

impl<State, Trigger> Plugin for SpellPlugin<State, Trigger>
where
    State: Sync + Send + Debug + Clone + Copy + Eq + Hash + 'static,
    Trigger: Sync + Send + 'static,
    Trigger: SpellTrigger,
{
    fn build(&self, app: &mut AppBuilder) {
        let spells = SystemSet::on_update(self.state)
            .label(SPELLS_LABEL)
            .with_system(spell_tracking.system())
            .with_system(spell_projectile_motion.system())
            .with_system(spell_projectile_collision.system())
            .with_system(spell_impact::<Trigger>.exclusive_system());

        app.add_event::<SpellImpactEvent>().add_system_set(spells);
    }
}
