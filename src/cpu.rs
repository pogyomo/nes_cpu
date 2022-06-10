#![allow(dead_code)]

mod fetch;
mod execute;
mod interrupt;

use crate::opcode::OPCODE_TABLE;
use crate::register::Register;
use crate::memory::Memory;

pub struct Cpu {
    reg: Register,
    mem: Box<dyn Memory>,
}

impl Cpu {
    pub fn new(mem: Box<dyn Memory>) -> Cpu {
        Cpu { reg: Register::new(), mem }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch_opcode();
        let info   = OPCODE_TABLE.get(&opcode).unwrap_or_else(|| {
            panic!("Invalid opcode: 0x{:x}", opcode);
        });
        let addr = self.fetch_address(info);

        self.execute(addr, &info.name, &info.mode);
    }
}
