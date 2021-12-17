use bevy::math::Size;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum CharacterClass {
    /// Warrior class
    // Roman
    // Normal
    Mars,
    /// Rogue class
    // Roman
    // Thin
    Pluto,
    /// Heavy class
    // Bible
    // Fat
    Mammon,
    /// Mage class
    // Greek
    // Fire
    Medea,
    /// Healer class
    // Eygpt
    Heka,
}

impl CharacterClass {
    pub fn size(&self) -> Size {
        Size::new(30., 30.)
    }
}
