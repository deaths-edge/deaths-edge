#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ArenaState {
    /// Waiting for game to start
    Waiting,
    /// Games started
    Ready,
}
