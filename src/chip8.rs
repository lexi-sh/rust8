use std::io::Read;

pub struct Cpu {
    registers: [u8; 16],
    memory: [u8; 4048],
    stack_pointer: [u8; 64],
    delay_timer: u8,
    sound_timer: u8,
    gfx: [[bool; 32]; 64],
    i: u8,
    pc: u16,
    sp: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            memory: [0; 4048],
            stack_pointer: [0; 64],
            delay_timer: 0,
            sound_timer: 0,
            gfx: [[false; 32]; 64],
            i: 0b00000000,
            pc: 0x200,
            sp: 0b00000000,
        }
    }
    
    pub fn run<T: Read>(&mut self, mut reader: T) {
        let mut buffer: Vec<u8> = Vec::new();
        for (index, byte) in reader.bytes().enumerate() {
            let b = byte.unwrap();
            self.memory[index + 512] = b;
        }
        
        loop {
            // emulate one cycle
            
            // if draw flag
                // draw graphics
                
            // set keys
        }
    }
    
    fn emulate_cycle(&self, opcode: u8) {
        
    }
    
    
    
}
