use crate::{print, println};

#[derive(Copy, Clone, Debug)]
pub struct CpuRam {
    pub ram_data: [u8; 0x10000],
}

pub fn ram_clear() -> CpuRam {
    let temp_ram = CpuRam {
        ram_data: [0; 0x10000],
    };
    return temp_ram;
}

pub fn test_ram(ram_d: CpuRam) {
    for (addr, data) in ram_d.ram_data.iter().enumerate() {
        if *data != 0 {
            crate::println!("{} at {}", addr, ram_d.ram_data[addr]);
        }
    }
}
