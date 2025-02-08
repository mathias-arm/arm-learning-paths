use core::fmt;

const DEFAULT_BASE_ADDRESS: usize = 0x1c09_0000;

pub struct Pl011 {
    base_address: *mut u8,
}

impl Pl011 {
    pub fn new(base_address: usize) -> Self {
        Pl011 {
            base_address: base_address as *mut u8,
        }
    }

    pub fn write_byte(&self, byte: u8) {
        while self.read_flag_register() & (1 << 5) != 0 {}
        unsafe {
            self.base_address.write_volatile(byte);
        }
        while self.read_flag_register() & (1 << 3) != 0 {}
    }

    fn read_flag_register(&self) -> u8 {
        unsafe { self.base_address.add(0x18).read_volatile() }
    }
}

impl Default for Pl011 {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_ADDRESS)
    }
}

impl fmt::Write for Pl011 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            self.write_byte(*c);
        }
        Ok(())
    }
}

unsafe impl Send for Pl011 {}
