use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterIndex(usize);

impl From<usize> for CharacterIndex {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl CharacterIndex {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
