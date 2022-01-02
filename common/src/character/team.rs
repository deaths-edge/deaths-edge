use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
pub enum Team {
    Red,
    Blue,
}
