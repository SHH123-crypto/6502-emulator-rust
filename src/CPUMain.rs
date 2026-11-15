use crate::ram;
use ram::CpuRam;

pub static mut RAM: CpuRam = CpuRam {
    ram_data: [0; 0x10000],
};

pub mod Bus {
    use super::CpuRam;
    use super::RAM;
    pub fn ret_ram_state() -> CpuRam {
        unsafe {
            return RAM;
        }
    }
    pub fn ram_read(addr: u16, mut _readOnly: bool, testram: &CpuRam) -> u8 {
        //this needs to read from ram
        return testram.ram_data[addr as usize]; //the testram parameter is temporary, delete it and use the RAM var
    }

    pub fn ram_write(addr: u16, data: u8, testram: &mut CpuRam) {
        //this needs to write to ram
        //unsafe {
        testram.ram_data[addr as usize] = data;
        //}
    }
}

mod mainCpu {
    use super::RAM;
}
