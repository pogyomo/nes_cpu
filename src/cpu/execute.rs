use super::Cpu;
use crate::opcode::{Mnemonic, AddressingMode};
use crate::register::Status;

impl Cpu {
    pub fn execute(&mut self, addr: u16, name: &Mnemonic, mode: &AddressingMode) {
        match name {
            Mnemonic::Adc => self.adc(addr),
            Mnemonic::And => self.and(addr),
            Mnemonic::Asl if *mode == AddressingMode::Accumulator => self.asl_acc(),
            Mnemonic::Asl => self.asl(addr),
            Mnemonic::Bcc => self.bcc(addr),
            Mnemonic::Bcs => self.bcs(addr),
            Mnemonic::Beq => self.beq(addr),
            Mnemonic::Bit => self.bit(addr),
            Mnemonic::Bmi => self.bmi(addr),
            Mnemonic::Bne => self.bne(addr),
            Mnemonic::Bpl => self.bpl(addr),
            Mnemonic::Brk => self.brk(),
            Mnemonic::Bvc => self.bvc(addr),
            Mnemonic::Bvs => self.bvs(addr),
            Mnemonic::Clc => self.clc(),
            Mnemonic::Cld => self.cld(),
            Mnemonic::Cli => self.cli(),
            Mnemonic::Clv => self.clv(),
            Mnemonic::Cmp => self.cmp(addr),
            Mnemonic::Cpx => self.cpx(addr),
            Mnemonic::Cpy => self.cpy(addr),
            Mnemonic::Dec => self.dec(addr),
            Mnemonic::Dex => self.dex(),
            Mnemonic::Dey => self.dey(),
            Mnemonic::Eor => self.eor(addr),
            Mnemonic::Inc => self.inc(addr),
            Mnemonic::Inx => self.inx(),
            Mnemonic::Iny => self.iny(),
            Mnemonic::Jmp => self.jmp(addr),
            Mnemonic::Jsr => self.jsr(addr),
            Mnemonic::Lda => self.lda(addr),
            Mnemonic::Ldx => self.ldx(addr),
            Mnemonic::Ldy => self.ldy(addr),
            Mnemonic::Lsr if *mode == AddressingMode::Accumulator => self.lsr_acc(),
            Mnemonic::Lsr => self.lsr(addr),
            Mnemonic::Nop => self.nop(),
            Mnemonic::Ora => self.ora(addr),
            Mnemonic::Pha => self.pha(),
            Mnemonic::Php => self.php(),
            Mnemonic::Pla => self.pla(),
            Mnemonic::Plp => self.plp(),
            Mnemonic::Rol if *mode == AddressingMode::Accumulator => self.rol_acc(),
            Mnemonic::Rol => self.rol(addr),
            Mnemonic::Ror if *mode == AddressingMode::Accumulator => self.ror_acc(),
            Mnemonic::Ror => self.ror(addr),
            Mnemonic::Rti => self.rti(),
            Mnemonic::Rts => self.rts(),
            Mnemonic::Sbc => self.sbc(addr),
            Mnemonic::Sec => self.sec(),
            Mnemonic::Sed => self.sed(),
            Mnemonic::Sei => self.sei(),
            Mnemonic::Sta => self.sta(addr),
            Mnemonic::Stx => self.stx(addr),
            Mnemonic::Sty => self.sty(addr),
            Mnemonic::Tax => self.tax(),
            Mnemonic::Tay => self.tay(),
            Mnemonic::Tsx => self.tsx(),
            Mnemonic::Txa => self.txa(),
            Mnemonic::Txs => self.txs(),
            Mnemonic::Tya => self.tya(),
        }
    }

    fn adc(&mut self, addr: u16) {
        let carry        = if self.reg.p.contains(Status::CARRY) { 1 } else { 0 };
        let value_to_add = self.mem.read_byte(addr).overflowing_add(carry);
        let result       = self.reg.a.overflowing_add(value_to_add.0);
        let is_carry     = value_to_add.1 | result.1;
        let is_overflow  = (self.reg.a >> 7) == (value_to_add.0 >> 7) &&
                           (self.reg.a >> 7) != (result.0       >> 7);
        self.reg.a = result.0;

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.set(Status::OVERFLOW, is_overflow);
        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn and(&mut self, addr: u16) {
        self.reg.a &= self.mem.read_byte(addr);

        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn asl_acc(&mut self) {
        let is_carry = self.reg.a >> 7 == 1;
        self.reg.a <<= 1;

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn asl(&mut self, addr: u16) {
        let value    = self.mem.read_byte(addr);
        let is_carry = value >> 7 == 1;
        let result   = value << 1;
        self.mem.write_byte(addr, result);

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.update_zero_and_negative(result);
    }

    fn bcc(&mut self, addr: u16) {
        self.branch(addr, !self.reg.p.contains(Status::CARRY));
    }

    fn bcs(&mut self, addr: u16) {
        self.branch(addr, self.reg.p.contains(Status::CARRY));
    }

    fn beq(&mut self, addr: u16) {
        self.branch(addr, self.reg.p.contains(Status::ZERO));
    }

    fn bit(&mut self, addr: u16) {
        self.reg.p.set(Status::NEGATIVE, self.mem.read_byte(addr) & 0b1000_0000 != 0);
        self.reg.p.set(Status::OVERFLOW, self.mem.read_byte(addr) & 0b0100_0000 != 0);
        self.reg.p.set(Status::ZERO,     self.mem.read_byte(addr) & self.reg.a  != 0);
    }

    fn bmi(&mut self, addr: u16) {
        self.branch(addr, self.reg.p.contains(Status::NEGATIVE));
    }

    fn bne(&mut self, addr: u16) {
        self.branch(addr, !self.reg.p.contains(Status::ZERO));
    }

    fn bpl(&mut self, addr: u16) {
        self.branch(addr, !self.reg.p.contains(Status::NEGATIVE));
    }

    fn brk(&mut self) {
        if !self.reg.p.contains(Status::INTERRUPT) {
            self.push_word(self.reg.pc);
            self.reg.p.insert(Status::BREAK);
            self.push_byte(self.reg.p.as_bits());

            self.reg.pc = self.mem.read_word(0xFFFE);
        }
    }

    fn bvc(&mut self, addr: u16) {
        self.branch(addr, !self.reg.p.contains(Status::OVERFLOW));
    }

    fn bvs(&mut self, addr: u16) {
        self.branch(addr, self.reg.p.contains(Status::OVERFLOW));
    }

    fn clc(&mut self) {
        self.reg.p.remove(Status::CARRY);
    }

    fn cld(&mut self) {
        self.reg.p.remove(Status::DECIMAL);
    }

    fn cli(&mut self) {
        self.reg.p.remove(Status::INTERRUPT);
    }

    fn clv(&mut self) {
        self.reg.p.remove(Status::OVERFLOW);
    }

    fn cmp(&mut self, addr: u16) {
        let result = self.reg.a.overflowing_add(self.mem.read_byte(addr));

        self.reg.p.set(Status::CARRY, !result.1);
        self.reg.p.update_zero_and_negative(result.0);
    }

    fn cpx(&mut self, addr: u16) {
        let result = self.reg.x.overflowing_add(self.mem.read_byte(addr));

        self.reg.p.set(Status::CARRY, !result.1);
        self.reg.p.update_zero_and_negative(result.0);
    }

    fn cpy(&mut self, addr: u16) {
        let result = self.reg.y.overflowing_add(self.mem.read_byte(addr));

        self.reg.p.set(Status::CARRY, !result.1);
        self.reg.p.update_zero_and_negative(result.0);
    }

    fn dec(&mut self, addr: u16) {
        let result = self.mem.read_byte(addr).wrapping_sub(1);
        self.mem.write_byte(addr, result);

        self.reg.p.update_zero_and_negative(result);
    }

    fn dex(&mut self) {
        self.reg.x = self.reg.x.wrapping_sub(1);

        self.reg.p.update_zero_and_negative(self.reg.x);
    }

    fn dey(&mut self) {
        self.reg.y = self.reg.y.wrapping_sub(1);

        self.reg.p.update_zero_and_negative(self.reg.y);
    }

    fn eor(&mut self, addr: u16) {
        self.reg.a ^= self.mem.read_byte(addr);

        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn inc(&mut self, addr: u16) {
        let result = self.mem.read_byte(addr).wrapping_add(1);
        self.mem.write_byte(addr, result);

        self.reg.p.update_zero_and_negative(result);
    } 

    fn inx(&mut self) {
        self.reg.x = self.reg.x.wrapping_add(1);

        self.reg.p.update_zero_and_negative(self.reg.x);
    }

    fn iny(&mut self) {
        self.reg.y = self.reg.y.wrapping_add(1);

        self.reg.p.update_zero_and_negative(self.reg.y);
    }

    fn jmp(&mut self, addr: u16) {
        self.reg.pc = addr;
    }

    fn jsr(&mut self, addr: u16) {
        self.push_word(self.reg.pc.wrapping_sub(1));

        self.reg.pc = addr;
    }

    fn lda(&mut self, addr: u16) {
        self.reg.a = self.mem.read_byte(addr);

        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn ldx(&mut self, addr: u16) {
        self.reg.x = self.mem.read_byte(addr);

        self.reg.p.update_zero_and_negative(self.reg.x);
    }

    fn ldy(&mut self, addr: u16) {
        self.reg.x = self.mem.read_byte(addr);

        self.reg.p.update_zero_and_negative(self.reg.x);
    }

    fn lsr_acc(&mut self) {
        let is_carry = self.reg.a & 0b0000_0001 == 0b0000_0001;
        self.reg.a >>= 1;

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn lsr(&mut self, addr: u16) {
        let value    = self.mem.read_byte(addr);
        let is_carry = value & 0b0000_0001 == 0b0000_0001;
        let result   = value >> 1;
        self.mem.write_byte(addr, result);

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.update_zero_and_negative(result);
    }

    fn nop(&mut self) {}

    fn ora(&mut self, addr: u16) {
        self.reg.a |= self.mem.read_byte(addr);

        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn pha(&mut self) {
        self.push_byte(self.reg.a);
    }

    fn php(&mut self) {
        self.push_byte(self.reg.p.as_bits());
    }

    fn pla(&mut self) {
        self.reg.a = self.pull_byte();

        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn plp(&mut self) {
        self.reg.p = Status::from_bits(self.pull_byte());
    }

    fn rol_acc(&mut self) {
        let is_carry = self.reg.a >> 7 == 1;
        let carry    = if self.reg.p.contains(Status::CARRY) { 1 } else { 0 };
        self.reg.a   = (self.reg.a << 1) + carry;

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn rol(&mut self, addr: u16) {
        let value    = self.mem.read_byte(addr);
        let is_carry = value >> 7 == 1;
        let carry    = if self.reg.p.contains(Status::CARRY) { 1 } else { 0 };
        let result   = (value << 1) + carry;
        self.mem.write_byte(addr, result);

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.update_zero_and_negative(result);
    }

    fn ror_acc(&mut self) {
        let is_carry = self.reg.a & 0b0000_0001 == 0b0000_0001;
        let carry    = if self.reg.p.contains(Status::CARRY) { 0b1000_0000 } else { 0 };
        self.reg.a   = (self.reg.a >> 1) + carry;

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn ror(&mut self, addr: u16) {
        let value    = self.mem.read_byte(addr);
        let is_carry = value & 0b0000_0001 == 0b0000_0001;
        let carry    = if self.reg.p.contains(Status::CARRY) { 0b1000_0000 } else { 0 };
        let result   = (value >> 1) + carry;
        self.mem.write_byte(addr, result);

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.update_zero_and_negative(result);
    }

    fn rti(&mut self) {
        self.reg.p  = Status::from_bits(self.pull_byte());
        self.reg.pc = self.pull_word();
    }

    fn rts(&mut self) {
        self.reg.p  = Status::from_bits(self.pull_byte());
        self.reg.pc = self.pull_word().wrapping_add(1);
    }

    fn sbc(&mut self, addr: u16) {
        let carry        = if self.reg.p.contains(Status::CARRY) { 0 } else { 1 };
        let value_to_sub = self.mem.read_byte(addr).overflowing_add(carry);
        let result       = self.reg.a.overflowing_sub(value_to_sub.0);
        let is_carry     = !(value_to_sub.1 | result.1);
        let is_overflow  = (self.reg.a >> 7) == (value_to_sub.0 >> 7) &&
                           (self.reg.a >> 7) != (result.0       >> 7);
        self.reg.a = result.0;

        self.reg.p.set(Status::CARRY, is_carry);
        self.reg.p.set(Status::OVERFLOW, is_overflow);
        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn sec(&mut self) {
        self.reg.p.insert(Status::CARRY);
    }

    fn sed(&mut self) {
        self.reg.p.insert(Status::DECIMAL);
    }

    fn sei(&mut self) {
        self.reg.p.insert(Status::INTERRUPT);
    }

    fn sta(&mut self, addr: u16) {
        self.mem.write_byte(addr, self.reg.a);
    }

    fn stx(&mut self, addr: u16) {
        self.mem.write_byte(addr, self.reg.x);
    }

    fn sty(&mut self, addr: u16) {
        self.mem.write_byte(addr, self.reg.y);
    }

    fn tax(&mut self) {
        self.reg.x = self.reg.a;

        self.reg.p.update_zero_and_negative(self.reg.x);
    }

    fn tay(&mut self) {
        self.reg.y = self.reg.a;

        self.reg.p.update_zero_and_negative(self.reg.y);
    }

    fn tsx(&mut self) {
        self.reg.x = self.reg.s;

        self.reg.p.update_zero_and_negative(self.reg.x);
    }

    fn txa(&mut self) {
        self.reg.a = self.reg.x;

        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn txs(&mut self) {
        self.reg.s = self.reg.x;
    }

    fn tya(&mut self) {
        self.reg.a = self.reg.y;

        self.reg.p.update_zero_and_negative(self.reg.a);
    }

    fn branch(&mut self, addr: u16, value: bool) {
        if value {
            self.reg.pc = addr;
        }
    }

    fn push_byte(&mut self, byte: u8) {
        self.mem.write_byte(self.reg.s as u16 + 0x0100, byte);
        self.reg.s = self.reg.s.wrapping_sub(1);
    }

    fn pull_byte(&mut self) -> u8 {
        self.reg.s = self.reg.s.wrapping_add(1);
        self.mem.read_byte(self.reg.s as u16 + 0x0100)
    }

    fn push_word(&mut self, word: u16) {
        let bytes = word.to_le_bytes();
        self.push_byte(bytes[0]);
        self.push_byte(bytes[1]);
    }

    fn pull_word(&mut self) -> u16 {
        let msb = self.pull_byte();
        let lsb = self.pull_byte();
        u16::from_le_bytes([lsb, msb])
    }
}
