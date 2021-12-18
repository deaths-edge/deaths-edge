use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};

use serde::{Deserialize, Serialize};

use crate::character::{CharacterClass, CharacterTeam};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ArenaPasscode(pub u64);

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
pub struct ArenaPermit {
    pub passcode: ArenaPasscode,
    pub class: CharacterClass,
    pub team: CharacterTeam,
}

impl ArenaPermit {
    pub fn new(passcode: ArenaPasscode, class: CharacterClass, team: CharacterTeam) -> Self {
        Self {
            passcode,
            class,
            team,
        }
    }
}

/// Sent by matchmaking server.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameRoster {
    permits: HashSet<ArenaPermit>,
    success: HashMap<SocketAddr, ArenaPermit>,
}

pub struct FraudulentPermit;

impl GameRoster {
    pub fn new(permits: HashSet<ArenaPermit>) -> Self {
        let permit_len = permits.len();
        Self {
            permits,
            success: HashMap::with_capacity(permit_len),
        }
    }

    pub fn drain(&mut self) -> impl Iterator<Item = (SocketAddr, ArenaPermit)> + '_ {
        self.success.drain()
    }

    pub fn apply_permit(
        &mut self,
        socket_address: SocketAddr,
        permit: &ArenaPermit,
    ) -> Result<(), FraudulentPermit> {
        let permit = self.permits.take(permit).ok_or(FraudulentPermit)?;
        self.success.insert(socket_address, permit);
        Ok(())
    }
}
