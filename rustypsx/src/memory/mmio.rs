use crate::memory;
use serde::{Deserialize, Serialize};

const EMPTY_BYTE: u8 = 0xFF;

#[derive(Serialize, Deserialize, Clone)]
pub struct Mmio {
}

impl Mmio {
    pub fn new() -> Self {
        Mmio {
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl memory::Addressable for Mmio {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            _ => EMPTY_BYTE
        }
    }

    fn write(&mut self, addr: u16, _value: u8) {
        match addr {
            _ => (),
        }
    }
}
