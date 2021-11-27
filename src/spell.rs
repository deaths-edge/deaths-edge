use bevy::prelude::*;

pub struct SpellIndex(usize);

#[derive(Bundle)]
pub struct SpellBundle {
    index: SpellIndex,
}
