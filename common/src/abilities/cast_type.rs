use std::time::Duration;

/// Ability requires casting duration.
#[derive(Debug)]
pub enum CastType {
    Instant,
    Cast(Duration),
    Channel(Duration),
}

impl Default for CastType {
    fn default() -> Self {
        Self::Instant
    }
}
