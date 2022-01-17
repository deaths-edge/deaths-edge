mod cast;
mod instant;

pub use cast::*;
pub use instant::*;

#[derive(Debug)]
pub struct AlreadyPerformed;
