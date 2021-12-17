/// Represents the current state of the arena.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ArenaState {
    /// Waiting for game to start
    Waiting,
    /// Games started
    Ready,
}

/// Represents whether players can be spawned.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SpawningState {
    Active,
    Deactive,
}
