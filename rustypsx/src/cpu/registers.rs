use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Registers {
    pub gpr: [u32; 32],
    pub hi: u32,
    pub lo: u32,
    pub pc: u32,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            gpr: [0; 32],
            hi: 0,
            lo: 0,
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.gpr = [0; 32];
        self.hi = 0;
        self.lo = 0;
        self.pc = 0x00;
    }
}
