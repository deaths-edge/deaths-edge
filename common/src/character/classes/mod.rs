pub mod mars;
pub mod medea;

use std::fmt;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, Component)]
pub enum Class {
    /// Melee: Damage + Survivability
    /// High sustained, medium health, escapes
    /// Warrior-like
    Mars,
    /// Melee: Damage + Utility
    /// High burst, low health, escapes, control
    /// Rogue-like
    Pluto,
    /// Melee: Survivability + Utility
    /// Low sustained, high health, control
    Mammon,
    /// Ranged: Damage + Survivability
    /// High sustained, high health, some escapes
    /// Warlock-like
    Nergal,
    /// Ranged: Damage + Utility
    /// High burst, low health, control, some escapes
    /// Mage-like
    Medea,
    /// Ranged: Survivability + Utility
    /// Redirects, medium health, escapes, control
    Janus,
    /// Healer: Healing + Survivability
    /// High health, close range, AOE
    Borvo,
    /// Healer: Healing + Utility
    /// Burst healing, control, damage
    /// Priest-like
    Heka,
    /// Healer: Utility + Survivability
    /// Low sustained healing, control, escapes
    /// Druid-like
    Rhea,
}

impl Class {
    pub fn cycle_right(self) -> Self {
        use Class::*;
        match self {
            Mars => Pluto,
            Pluto => Mammon,
            Mammon => Nergal,
            Nergal => Medea,
            Medea => Janus,
            Janus => Borvo,
            Borvo => Heka,
            Heka => Rhea,
            Rhea => Mars,
        }
    }

    pub fn cycle_left(self) -> Self {
        use Class::*;
        match self {
            Pluto => Mars,
            Mammon => Pluto,
            Nergal => Mammon,
            Medea => Nergal,
            Janus => Medea,
            Borvo => Janus,
            Heka => Borvo,
            Rhea => Heka,
            Mars => Rhea,
        }
    }

    pub fn color(&self) -> Color {
        Color::BLUE
    }
}

impl Class {
    pub fn as_str(&self) -> &'static str {
        use Class::*;
        match self {
            Mars => "Mars",
            Pluto => "Pluto",
            Mammon => "Mammon",
            Nergal => "Nergal",
            Medea => "Medea",
            Janus => "Janus",
            Borvo => "Borvo",
            Heka => "Heka",
            Rhea => "Rhea",
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}