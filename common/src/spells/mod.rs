mod cast;
mod impact;
mod index;
pub mod instances;
mod marker;
mod projectiles;
mod source;
mod target;
mod utilities;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;
use heron::{prelude::*, rapier_plugin::PhysicsWorld};

pub use cast::*;
pub use impact::*;
pub use marker::*;
pub use projectiles::*;
pub use source::*;
pub use target::*;
pub use utilities::*;

use crate::character::{CharacterMarker, LastCastInstant};

pub struct SpellPlugin<T> {
    state: T,
}

impl<T> SpellPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

pub const SPELLS_LABEL: &str = "spells";

impl<T> Plugin for SpellPlugin<T>
where
    T: Sync + Send + Debug + Clone + Copy + Eq + Hash + 'static,
{
    fn build(&self, app: &mut AppBuilder) {
        let spells = SystemSet::on_update(self.state)
            .label(SPELLS_LABEL)
            .with_system(spell_tracking.system())
            .with_system(spell_projectile_motion.system())
            .with_system(spell_projectile_collision.system())
            .with_system(spell_impact.exclusive_system());

        app.add_event::<SpellImpactEvent>().add_system_set(spells);
    }
}
