use bevy::prelude::*;

use crate::abilities::Target;

#[derive(Default, Debug, Clone, Copy, Component)]
pub struct OptionalTarget(pub Option<Target>);

impl From<Target> for OptionalTarget {
    fn from(target: Target) -> Self {
        Self(Some(target))
    }
}
