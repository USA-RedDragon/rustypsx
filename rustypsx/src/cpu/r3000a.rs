use crate::{cpu::registers, memory, memory::Addressable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct R3000A {
    pub registers: registers::Registers,
    pub halted: bool,
    pub stopped: bool,
}

impl R3000A {
    pub fn new() -> Self {
        R3000A { registers: registers::Registers::new(), halted: false, stopped: false }
    }

    pub fn step(&mut self, mmio: &mut memory::mmio::Mmio) -> u8 {
        let opcode = mmio.read(self.registers.pc);
        self.registers.pc += 1;
        self.execute(opcode, mmio)
    }

    fn execute(&mut self, opcode: u8, _mmio: &mut memory::mmio::Mmio) -> u8 {
        match opcode {
            _ => panic!("Opcode {:02X} not implemented", opcode),
        }
    }
}
