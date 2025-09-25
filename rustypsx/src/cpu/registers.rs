use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Registers {
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0x00;
    }
}
