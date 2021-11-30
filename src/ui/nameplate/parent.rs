use crate::character::CharacterIndex;

#[derive(Debug)]
pub struct NameplateParent(CharacterIndex);

impl From<CharacterIndex> for NameplateParent {
    fn from(value: CharacterIndex) -> Self {
        Self(value)
    }
}

impl PartialEq<CharacterIndex> for NameplateParent {
    fn eq(&self, other: &CharacterIndex) -> bool {
        self.0 == *other
    }
}
