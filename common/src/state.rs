/// Represents the current state of the arena.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ArenaState {
    /// Arena inactive
    Inactive,
    /// Waiting for game to start
    Waiting,
    /// Games started
    Ready,
}
