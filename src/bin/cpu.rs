
struct registers{
    a:u8, //accumulator
    f:u8, //flags Z N H C  (4 bottom bits are not used)
    b:u8,
    c:u8,
    e:u8,
    d:u8,
    h:u8,
    l:u8,
    sp:u16,
    pc:u16
}

impl registers {
    pub fn init()->Self{
        Self { a: 0x01, f: 0xB0, b: 0x00, c: 0x13, e: 0xD8, d: 0x00, h: 0x01, l: 0x4d, sp: 0xFFFE, pc: 0x0100 }
    }

    fn get_bc(&self)->u16{
        ((self.b as u16)<<8) | (self.c as u16)
    }
    fn set_bc(&mut self,value:u16){
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self)->u16{
        ((self.d as u16) << 8) | (self.e as u16)
    }
    fn set_de(&mut self, value:u16){
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self)->u16{
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_hl(&mut self,value:u16){
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    //get flags of the f register
    // --- FLAG GETTERS ---
    fn get_z_flag(&self) -> bool {
        (self.f & 0x80) != 0 // Bit 7
    }
    fn get_n_flag(&self) -> bool {
        (self.f & 0x40) != 0 // Bit 6
    }
    fn get_h_flag(&self) -> bool {
        (self.f & 0x20) != 0 // Bit 5 (Note: 0x20, not 0x10)
    }
    fn get_c_flag(&self) -> bool {
        (self.f & 0x10) != 0 // Bit 4 (Note: 0x10, not 0x01)
    }

    // --- FLAG SETTERS ---
    fn set_z_flag(&mut self, v: bool) {
        if v {
            self.f |= 0x80; // Turn bit 7 ON
        } else {
            self.f &= !0x80; // Turn bit 7 OFF
        }
    }

    fn set_n_flag(&mut self, v: bool) {
        if v {
            self.f |= 0x40; // Turn bit 6 ON
        } else {
            self.f &= !0x40; // Turn bit 6 OFF
        }
    }

    fn set_h_flag(&mut self, v: bool) {
        if v {
            self.f |= 0x20; // Turn bit 5 ON
        } else {
            self.f &= !0x20; // Turn bit 5 OFF
        }
    }

    fn set_c_flag(&mut self, v: bool) {
        if v {
            self.f |= 0x10; // Turn bit 4 ON
        } else {
            self.f &= !0x10; // Turn bit 4 OFF
        }
    }

    //other bits control:
    // --- AF REGISTER PAIR ---
    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        // Game Boy rule: Lower 4 bits of F are always 0
        self.f = ((value & 0xFF) & 0xF0) as u8;
    }

    // --- INDIVIDUAL REGISTER GETTERS & SETTERS ---
    // Accumulator (A)
    fn get_a(&self) -> u8 { self.a }
    fn set_a(&mut self, val: u8) { self.a = val; }

    // Stack Pointer (SP) - 16-bit
    fn get_sp(&self) -> u16 { self.sp }
    fn set_sp(&mut self, val: u16) { self.sp = val; }

    // Program Counter (PC) - 16-bit
    fn get_pc(&self) -> u16 { self.pc }
    fn set_pc(&mut self, val: u16) { self.pc = val; }

    // (Optional) Individual general-purpose 8-bit registers if needed:
    fn get_b(&self) -> u8 { self.b }
    fn set_b(&mut self, val: u8) { self.b = val; }
    
    fn get_c(&self) -> u8 { self.c }
    fn set_c(&mut self, val: u8) { self.c = val; }

    fn get_d(&self) -> u8 { self.d }
    fn set_d(&mut self, val: u8) { self.d = val; }

    fn get_e(&self) -> u8 { self.e }
    fn set_e(&mut self, val: u8) { self.e = val; }

    fn get_h(&self) -> u8 { self.h }
    fn set_h(&mut self, val: u8) { self.h = val; }

    fn get_l(&self) -> u8 { self.l }
    fn set_l(&mut self, val: u8) { self.l = val; }






    // ALU

    fn add_to_a(&mut self,val:u8){
        
    }

}