use crate::character::CharacterIndex;

#[derive(Debug, Clone, Copy)]
pub struct SpellTarget(CharacterIndex);

impl From<CharacterIndex> for SpellTarget {
    fn from(value: CharacterIndex) -> Self {
        Self(value)
    }
}

impl PartialEq<CharacterIndex> for SpellTarget {
    fn eq(&self, other: &CharacterIndex) -> bool {
        self.0 == *other
    }
}
