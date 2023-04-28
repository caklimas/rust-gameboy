use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IndexRegisters {
    pub x: u16,
    pub y: u16,
}
