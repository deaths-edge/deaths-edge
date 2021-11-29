use crate::{character::CharacterIndex, spells::SpellTarget};

pub struct EffectTarget(CharacterIndex);

impl From<CharacterIndex> for EffectTarget {
    fn from(value: CharacterIndex) -> Self {
        Self(value)
    }
}

impl Into<CharacterIndex> for EffectTarget {
    fn into(self) -> CharacterIndex {
        self.0
    }
}

impl From<SpellTarget> for EffectTarget {
    fn from(value: SpellTarget) -> Self {
        let value: CharacterIndex = value.into();
        value.into()
    }
}

impl PartialEq<CharacterIndex> for EffectTarget {
    fn eq(&self, other: &CharacterIndex) -> bool {
        self.0 == *other
    }
}
