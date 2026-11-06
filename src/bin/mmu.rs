//this file represent the memory menipulations ROM + RAM + VRAM 


struct EmulatorMemory{
    rom:[u8;0x8000], //32KB
    wram:[u8;0x2000], //8KB
    hram:[u8;0x007F], //HIGH RAM 127bits
    io:[u8;0x0080], // 128bits
    vram:[u8;0x2000] //8KB

}



impl EmulatorMemory {
    fn init()->Self{
        Self{
            rom:[0;0x8000],
            wram:[0;0x2000],
            hram:[0;0x007F],
            io:[0;0x0080],
            vram:[0;0x2000]
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => {
                // Writing to ROM does not change ROM data. 
                // Later, this is where you send commands to a Memory Bank Controller (MBC)!
            }
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value, // Write to echo writes to WRAM
            0xFE00..=0xFE9F => {} // OAM write placeholder
            0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => {}          // Interrupt Enable write placeholder
            _ => {}               // Ignore writes to unmapped memory
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize], // Echo RAM mirrors WRAM
            0xFE00..=0xFE9F => 0x00, // OAM placeholder for now
            0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => 0x00,          // Interrupt Enable register placeholder
            _ => 0xFF,               // Unmapped memory returns 0xFF
        }
    }

}