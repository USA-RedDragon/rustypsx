use crate::memory;
use serde::{Deserialize, Serialize};

const EMPTY_BYTE: u8 = 0xFF;

const USER_MEMORY_START: u32 = 0x00000000;
const USER_MEMORY_LENGTH: usize = 0x80000000; // 2 GB
const USER_MEMORY_END: u32 = USER_MEMORY_START + USER_MEMORY_LENGTH as u32 - 1;

const CACHED_KERNEL_MEMORY_START: u32 = 0x80000000;
const CACHED_KERNEL_MEMORY_LENGTH: usize = 0x20000000; // 512 MB
const CACHED_KERNEL_MEMORY_END: u32 = CACHED_KERNEL_MEMORY_START + CACHED_KERNEL_MEMORY_LENGTH as u32 - 1;

const UNCACHED_KERNEL_MEMORY_START: u32 = 0xA0000000;
const UNCACHED_KERNEL_MEMORY_LENGTH: usize = 0x20000000; // 512 MB
const UNCACHED_KERNEL_MEMORY_END: u32 = UNCACHED_KERNEL_MEMORY_START + UNCACHED_KERNEL_MEMORY_LENGTH as u32 - 1;

const VIRTUAL_MEMORY_START: u32 = 0xC0000000;
const VIRTUAL_MEMORY_LENGTH: usize = 0x40000000; // 1 GB
const VIRTUAL_MEMORY_END: u32 = VIRTUAL_MEMORY_START - 1 + VIRTUAL_MEMORY_LENGTH as u32;

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
    fn read(&self, addr: u32) -> u8 {
        match addr {
            USER_MEMORY_START..=USER_MEMORY_END => EMPTY_BYTE,
            CACHED_KERNEL_MEMORY_START..=CACHED_KERNEL_MEMORY_END => EMPTY_BYTE,
            UNCACHED_KERNEL_MEMORY_START..=UNCACHED_KERNEL_MEMORY_END => EMPTY_BYTE,
            VIRTUAL_MEMORY_START..=VIRTUAL_MEMORY_END => EMPTY_BYTE,
        }
    }

    fn write(&mut self, addr: u32, _value: u8) {
        match addr {
            USER_MEMORY_START..=USER_MEMORY_END => {}
            CACHED_KERNEL_MEMORY_START..=CACHED_KERNEL_MEMORY_END => {}
            UNCACHED_KERNEL_MEMORY_START..=UNCACHED_KERNEL_MEMORY_END => {}
            VIRTUAL_MEMORY_START..=VIRTUAL_MEMORY_END => {}
        }
    }
}
