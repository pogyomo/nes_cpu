use super::Cpu;
use crate::opcode::{OpcodeInfo, AddressingMode};

impl Cpu {
    pub fn fetch_opcode(&mut self) -> u8 {
        self.fetch_byte()
    }

    pub fn fetch_address(&mut self, info: &OpcodeInfo) -> u16 {
        match info.mode {
            AddressingMode::Accumulator | AddressingMode::Implied => 0,
            AddressingMode::Absolute  => self.fetch_absolute_with_index(0),
            AddressingMode::AbsoluteX => self.fetch_absolute_with_index(self.reg.x),
            AddressingMode::AbsoluteY => self.fetch_absolute_with_index(self.reg.y),
            AddressingMode::Immediate => self.fetch_immediate(),
            AddressingMode::Indirect  => self.fetch_indirect(),
            AddressingMode::IndirectX => self.fetch_indirect_with_index((self.reg.x, 0)),
            AddressingMode::IndirectY => self.fetch_indirect_with_index((0, self.reg.y)),
            AddressingMode::Relative  => self.fetch_relative(),
            AddressingMode::ZeroPage  => self.fetch_zeropage_with_index(0),
            AddressingMode::ZeroPageX => self.fetch_zeropage_with_index(self.reg.x),
            AddressingMode::ZeroPageY => self.fetch_zeropage_with_index(self.reg.y),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let ret = self.mem.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        ret
    }

    fn fetch_word(&mut self) -> u16 {
        let lsb = self.fetch_byte();
        let msb = self.fetch_byte();
        u16::from_le_bytes([lsb, msb])
    }

    fn fetch_absolute_with_index(&mut self, index: u8) -> u16 {
        self.fetch_word().wrapping_add(index as u16)
    }

    fn fetch_immediate(&mut self) -> u16 {
        let addr = self.reg.pc;
        let _ = self.fetch_byte();
        addr
    }

    fn fetch_indirect(&mut self) -> u16 {
        let addr = self.fetch_word();
        self.mem.read_word(addr)
    }

    fn fetch_indirect_with_index(&mut self, index: (u8, u8)) -> u16 {
        let addr = self.fetch_byte().wrapping_add(index.0) as u16;
        self.mem.read_word(addr).wrapping_add(index.1 as u16)
    }

    fn fetch_relative(&mut self) -> u16 {
        let offset = self.fetch_byte();
        if offset >> 7 == 1 {
            self.reg.pc.wrapping_sub((!offset).wrapping_add(1) as u16)
        } else {
            self.reg.pc.wrapping_add(offset as u16)
        }
    }

    fn fetch_zeropage_with_index(&mut self, index: u8) -> u16 {
        self.fetch_byte().wrapping_add(index) as u16
    }
}
