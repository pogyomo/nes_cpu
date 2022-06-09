//! Provide trait that represent memoory

/// An trait that represent memory
pub trait Memory {
    /// Read 8bit value from given address
    fn read_byte(&self, addr: u16) -> u8;

    /// Write 8bit value to given address
    fn write_byte(&mut self, addr: u16, value: u8);

    /// Read 16bit value from given address
    fn read_word(&self, addr: u16) -> u16 {
        let lsb = self.read_byte(addr);
        let msb = self.read_byte(addr);
        u16::from_le_bytes([lsb, msb])
    }

    /// Write 16bit value to given address
    fn write_word(&mut self, addr: u16, value: u16) {
        let bytes = value.to_le_bytes();
        self.write_byte(addr.wrapping_add(0), bytes[0]);
        self.write_byte(addr.wrapping_add(1), bytes[1]);
    }
}
