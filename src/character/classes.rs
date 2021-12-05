#[derive(Clone, Copy)]
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
    pub fn size(&self) -> (f32, f32) {
        (30., 30.)
    }
}
