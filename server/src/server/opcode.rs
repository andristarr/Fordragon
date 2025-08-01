use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash, Default)]
pub enum OpCode {
    #[default]
    Unset,
    Moved,
    Move,
    Spawn,
    Enter,
    Enown,
}
