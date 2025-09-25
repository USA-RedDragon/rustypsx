use crate::cpu;
use crate::display;
use crate::memory;

use serde::{Deserialize, Serialize};

use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct PS1 {
    cpu: cpu::R3000A,
    mmio: memory::mmio::Mmio,
    #[serde(skip, default)]
    breakpoints: HashSet<u16>,
}

impl Clone for PS1 {
    fn clone(&self) -> Self {
        PS1 {
            cpu: self.cpu.clone(),
            mmio: self.mmio.clone(),
            breakpoints: self.breakpoints.clone(),
        }
    }
}

impl PS1 {
    pub fn new() -> Self {
        PS1 {
            cpu: cpu::R3000A::new(),
            mmio: memory::mmio::Mmio::new(),
            breakpoints: HashSet::new(),
        }
    }

    pub fn reset(&mut self) {
        self.mmio.reset();
        self.cpu.halted = false;
        self.cpu.stopped = false;
        self.cpu.registers.reset();
    }

    pub fn get_current_frame(&mut self) -> [u8; display::WIDTH as usize * display::HEIGHT as usize * 4] {
        [0; display::WIDTH as usize * display::HEIGHT as usize * 4]
    }

    pub fn step_instruction(&mut self, _collect_audio: bool) -> (bool, u8) {
        let pc = self.cpu.registers.pc;
        if self.breakpoints.contains(&pc) {
            return (true, 0);
        }

        let cycles = self.cpu.step(&mut self.mmio);
        (false, cycles)
    }

    pub fn run_until_frame(&mut self, collect_audio: bool) -> ([u8; display::WIDTH as usize * display::HEIGHT as usize * 4], bool) {
        let mut cpu_cycles_this_frame = 0u32;
        const MAX_CYCLES_PER_FRAME: u32 = 1000; // Placeholder value
        let frame = [0; display::WIDTH as usize * display::HEIGHT as usize * 4];
        loop {
            let (breakpoint_hit, cycles) = self.step_instruction(collect_audio);
            cpu_cycles_this_frame += cycles as u32;
            
            if breakpoint_hit {
                // Breakpoint hit - return current frame and indicate breakpoint hit
                return (frame, true);
            }
            
            if false { // self.ppu.frame_ready() {
                return (frame, false);
            }
            
            if cpu_cycles_this_frame >= MAX_CYCLES_PER_FRAME {
                return (frame, false);
            }
        }
    }

    pub fn get_cpu_registers(&self) -> &cpu::registers::Registers {
        &self.cpu.registers
    }

    pub fn add_breakpoint(&mut self, address: u16) {
        self.breakpoints.insert(address);
    }

    pub fn remove_breakpoint(&mut self, address: u16) {
        self.breakpoints.remove(&address);
    }

    pub fn get_breakpoints(&self) -> &HashSet<u16> {
        &self.breakpoints
    }
}