use core::ops::Not;

pub static mosCPU: cpuReg = cpuReg {
    a: 0x00,
    x: 0x00,
    y: 0x00,
    stkp: 0x00,
    pc: 0x0000,
    status: 0x00,
};

#[derive(Copy, Clone, Debug)]
pub struct CpuRam {
    pub ram_data: [u8; 0x10000],
}

impl CpuRam {
    pub fn ram_clear(&mut self) {
        self.ram_data = [0; 0x10000];
    }
}
pub static mut RAM: CpuRam = CpuRam {
    ram_data: [0; 0x10000],
};

pub mod Bus {

    use super::RAM;

    pub fn ram_read(addr: u16) -> u8 {
        //this needs to read from ram
        unsafe { RAM.ram_data[addr as usize] }
    }

    pub fn ram_write(addr: u16, data: u8) {
        //this needs to write to ram
        //unsafe {
        unsafe {
            RAM.ram_data[addr as usize] = data;
        }
        //}
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flags6502 {
    C = 1 << 0, // Carry Bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable Interrupts
    D = 1 << 3, // Decimal Mode
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}

impl Not for Flags6502 {
    type Output = u8;

    fn not(self) -> Self::Output {
        !(self as u8)
    }
}

pub struct cpuReg {
    a: u8,          //A register
    x: u8,          //X register
    y: u8,          //Y register
    stkp: u8,       //Stack Pointer (Points to location on Bus)
    pc: u16,        //Program Counter
    pub status: u8, //Status register
}

impl cpuReg {
    pub fn get_flag(&self, flag: Flags6502) -> bool {
        self.status & flag as u8 != 0
    }

    pub fn set_flag(&mut self, flag: Flags6502, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !flag as u8;
        }
    }

    pub fn cpu_reset(&mut self) {
        let addr_abs: u16 = 0xFFFC;
        let lo: u16 = Bus::ram_read(addr_abs + 0).into();
        let hi: u16 = Bus::ram_read(addr_abs + 1).into();
        self.pc = (hi << 8) | lo;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00;
    }
}
