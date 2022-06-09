#![allow(dead_code)]

use std::ops::{BitAnd, BitOr, BitXor};

pub struct Status {
    bits: u8,
}

impl BitAnd for Status {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { bits: self.bits & rhs.bits }
    }
}

impl BitOr  for Status {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { bits: self.bits | rhs.bits }
    }
}

impl BitXor for Status {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { bits: self.bits ^ rhs.bits }
    }
}

impl Status {
    pub const CARRY:     Self = Self { bits: 0b0000_0001 };
    pub const ZERO:      Self = Self { bits: 0b0000_0010 };
    pub const INTERRUPT: Self = Self { bits: 0b0000_0100 };
    pub const DECIMAL:   Self = Self { bits: 0b0000_1000 };
    pub const BREAK:     Self = Self { bits: 0b0001_0000 };
    pub const ALWAYS:    Self = Self { bits: 0b0010_0000 };
    pub const OVERFLOW:  Self = Self { bits: 0b0100_0000 };
    pub const NEGATIVE:  Self = Self { bits: 0b1000_0000 };

    // Return bits that represent Status
    pub const fn as_bits(&self) -> u8 {
        self.bits | Status::ALWAYS.bits
    }

    // Construct Status from given bits
    pub const fn from_bits(bits: u8) -> Status {
        Status { bits: bits | Status::ALWAYS.as_bits() }
    }

    /// Set flags depend on given value
    pub fn set(&mut self, bits: Self, value: bool) {
        if value {
            self.bits |=  bits.as_bits();
        } else {
            self.bits &= !bits.as_bits();
        }
    }

    // Insert flags
    pub fn insert(&mut self, bits: Self) {
        self.set(bits, true);
    }

    // Remove flags
    pub fn remove(&mut self, bits: Self) {
        self.set(bits, false);
    }

    // If flags is on, then return true
    pub fn contains(&self, other: Self) -> bool {
        self.as_bits() & other.as_bits() != 0
    }

    // Change flags depend on given value
    pub fn update_zero_and_negative(&mut self, value: u8) {
        self.set(Status::ZERO,     value      == 0);
        self.set(Status::NEGATIVE, value >> 7 != 0);
    }
}

pub struct Register {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: Status,
}

impl Register {
    pub fn new() -> Register {
        Register { a: 0, x: 0, y: 0, pc: 0, s: 0, p: Status::from_bits(0) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_status_bit_operator() {
        let and = Status::from_bits(0b0000_1111)
                & Status::from_bits(0b1111_0000);
        let or  = Status::from_bits(0b0000_1111)
                | Status::from_bits(0b1111_0000);
        let xor = Status::from_bits(0b0000_1111)
                ^ Status::from_bits(0b1111_0000);

        assert_eq!(and.as_bits(), 0b0010_0000);
        assert_eq!(or.as_bits(),  0b1111_1111);
        assert_eq!(xor.as_bits(), 0b1111_1111);
    }
}
