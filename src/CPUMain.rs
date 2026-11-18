use crate::ram;
use ram::CpuRam;




pub static mut mosCPU: cpuReg = cpuReg {
    a = 0x00,
    x = 0x00,  
    y = 0x00,  
    stkp = 0x00,  
    pc = 0x0000, 
    status = 0x00, 
}


pub static mut RAM: CpuRam = CpuRam {
    ram_data: [0; 0x10000],
};

pub mod Bus {
    use super::CpuRam;
    use super::RAM;

    pub fn ram_read(addr: u16) -> u8 {
        //this needs to read from ram
        return RAM.ram_data[addr as usize]; //the testram parameter is temporary, delete it and use the RAM var
    }

    pub fn ram_write(addr: u16, data: u8) {
        //this needs to write to ram
        //unsafe {
        RAM.ram_data[addr as usize] = data;
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

pub struct cpuReg {
        a: u8, //A register
        x: u8, //X register
        y: u8,  //Y register
        stkp: u8,  //Stack Pointer (Points to location on Bus)
        pc: u16, //Program Counter
        status: u8, //Status register       
}
//implement the Bus trait

impl cpuReg {
    use super::RAM;

    pub fn get_flag(&self, flag: Flags6502) {
        self.status & flag as u8 != 0
    }

    pub fn set_flag(&mut self, flag: Flags6502, value: bool) {

    }

    pub fn ram_reset(&mut self) {
        let addr_abs: u16 = 0xFFFC;
        let lo: u16 = read(addr_abs + 0);
        let hi: u16 = read(addr_abs + 1);  
        self.pc = (hi << 8) | lo;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00;
    }


}
