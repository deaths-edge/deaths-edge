use super::CharacterIndex;

#[derive(Default, Debug, Clone, Copy)]
pub struct CharacterTarget {
    // TODO: Remove pub
    pub target: Option<CharacterIndex>,
}

impl From<CharacterIndex> for CharacterTarget {
    fn from(index: CharacterIndex) -> Self {
        Self {
            target: Some(index),
        }
    }
}

impl CharacterTarget {
    pub fn set_index(&mut self, character: CharacterIndex) -> &mut Self {
        self.target = Some(character);
        self
    }

    pub fn deselect(&mut self) -> &mut Self {
        self.target = None;
        self
    }

    pub fn index(&self) -> Option<CharacterIndex> {
        self.target
    }
}
