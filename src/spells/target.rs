use crate::character::CharacterIndex;

#[derive(Debug, Clone, Copy)]
pub struct SpellTarget(CharacterIndex);

impl From<CharacterIndex> for SpellTarget {
    fn from(value: CharacterIndex) -> Self {
        Self(value)
    }
}

impl Into<CharacterIndex> for SpellTarget {
    fn into(self) -> CharacterIndex {
        self.0
    }
}

impl PartialEq<CharacterIndex> for SpellTarget {
    fn eq(&self, other: &CharacterIndex) -> bool {
        self.0 == *other
    }
}
