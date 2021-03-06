use std::collections::{HashMap, HashSet};

use bevy_networking_turbulence::ConnectionHandle;
use serde::{Deserialize, Serialize};

use crate::character::{Class, Team};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ArenaPasscode(pub u64);

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
pub struct ArenaPermit {
    pub passcode: ArenaPasscode,
    pub class: Class,
    pub team: Team,
}

/// Sent by matchmaking server.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameRoster {
    permits: HashSet<ArenaPermit>,
    success: HashMap<ConnectionHandle, ArenaPermit>,
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

    pub fn drain(&mut self) -> impl Iterator<Item = (ConnectionHandle, ArenaPermit)> + '_ {
        self.success.drain()
    }

    pub fn apply_permit(
        &mut self,
        connection_handle: ConnectionHandle,
        permit: &ArenaPermit,
    ) -> Result<(), FraudulentPermit> {
        let permit = self.permits.take(permit).ok_or(FraudulentPermit)?;
        self.success.insert(connection_handle, permit);
        Ok(())
    }
}
