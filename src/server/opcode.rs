use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OpCode {
    Movement,
    Auth,
    Existence,
    Spawn,
}
