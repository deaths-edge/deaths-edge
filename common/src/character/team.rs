use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Team {
    Red,
    Blue,
}
