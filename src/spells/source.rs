use crate::character::CharacterIndex;

#[derive(Debug, Clone, Copy)]
pub struct SpellSource(CharacterIndex);

impl From<CharacterIndex> for SpellSource {
    fn from(value: CharacterIndex) -> Self {
        Self(value)
    }
}

impl PartialEq<CharacterIndex> for SpellSource {
    fn eq(&self, other: &CharacterIndex) -> bool {
        self.0 == *other
    }
}
