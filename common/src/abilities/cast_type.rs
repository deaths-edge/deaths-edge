use std::time::Duration;

/// Ability requires casting duration.
pub enum CastType {
    Instant,
    Cast(Duration),
    Channel(Duration),
}
