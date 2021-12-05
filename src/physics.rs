use std::ops::{Deref, DerefMut};

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    character::CharacterMarker,
    environment::EnvironmentMarker,
    spells::{SpellImpactEvent, SpellMarker, SpellProjectileMarker, SpellTarget},
};
