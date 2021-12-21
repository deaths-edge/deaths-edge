use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterIndex(pub u8);

impl CharacterIndex {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
