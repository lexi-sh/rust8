use std::io::Read;

pub struct Cpu {
    v: [u8; 16],
    memory: [u8; 4048],
    stack: [u16; 64],
    delay_timer: u8,
    sound_timer: u8,
    gfx: [[bool; 32]; 64],
    i: u16,
    pc: u16,
    sp: u16,
    keys: [bool; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            memory: [0; 4048],
            stack: [0; 64],
            delay_timer: 0,
            sound_timer: 0,
            gfx: [[false; 32]; 64],
            i: 0b00000000,
            pc: 0x200,
            sp: 0b00000000,
            keys: [false; 16],
        }
    }
    
    pub fn run<T: Read>(&mut self, reader: T) {
        for (index, byte) in reader.bytes().enumerate() {
            let b = byte.unwrap();
            self.memory[index + 0x200] = b;
        }
        
        loop {
            // emulate one cycle
            self.emulate_cycle();
            // if draw flag
                // draw graphics
                
            // set keys
        }
    }
    
    fn emulate_cycle(&mut self) {
        let opcode: u16 = 
            ((self.memory[self.pc as usize]) as u16) << 8 | 
            (self.memory[(self.pc + 1) as usize]) as u16;
        
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0xFFF {
                    0x00EE => self.return_from_subroutine(),
                    _ => {}
                }
            },
            0x1000 => self.jump_to_address(opcode),
            0x2000 => self.call_subroutine(opcode),
            0x3000 => self.skip_if_equals_constant(opcode),
            0x4000 => self.skip_if_not_equals_constant(opcode),
            0x5000 => {
                match opcode & 0x000F {
                    0x0000 => self.skip_if_equals_v(opcode),
                    _ => {},
                }
            },
            0x6000 => self.set_register_constant(opcode),
            0x7000 => self.add_to_register(opcode),
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => self.set_register_value(opcode),
                    0x0001 => self.set_register_or(opcode),
                    0x0002 => self.set_register_and(opcode),
                    0x0003 => self.set_register_xor(opcode),
                    0x0004 => self.add(opcode),
                    _ => {},
                }
            },
            0x9000 => {
                match opcode & 0x000F {
                    0x0000 => self.skip_if_not_equals_v(opcode),
                    _ => {},
                }
            },
            0xA000 => self.set_i_to_opcode(opcode),
            _ => {},
        }
        
        self.pc += 2;
    }
    
    /*
        0NNN	Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
        00E0	Clears the screen.
DONE    00EE	Returns from a subroutine.
DONE    1NNN	Jumps to address NNN.
DONE    2NNN	Calls subroutine at NNN.
DONE    3XNN	Skips the next instruction if VX equals NN.
DONE    4XNN	Skips the next instruction if VX doesn't equal NN.
DONE    5XY0	Skips the next instruction if VX equals VY.
DONE    6XNN	Sets VX to NN. 
DONE    7XNN	Adds NN to VX.
DONE    8XY0	Sets VX to the value of VY.
DONE    8XY1	Sets VX to VX or VY.
DONE    8XY2	Sets VX to VX and VY.
DONE    8XY3	Sets VX to VX xor VY.
        8XY4	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
        8XY5	VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
        8XY6	Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.[2]
        8XY7	Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
        8XYE	Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.[2]
DONE    9XY0	Skips the next instruction if VX doesn't equal VY.
DONE    ANNN	Sets I to the address NNN.
        BNNN	Jumps to the address NNN plus V0.
        CXNN	Sets VX to the result of a bitwise and operation on a random number and NN.
        DXYN	Sprites stored in memory at location in index register (I), 8bits wide. Wraps around the screen. If when drawn, clears a pixel, register VF is set to 1 otherwise it is zero. All drawing is XOR drawing (i.e. it toggles the screen pixels). Sprites are drawn starting at position VX, VY. N is the number of 8bit rows that need to be drawn. If N is greater than 1, second line continues at position VX, VY+1, and so on.
        EX9E	Skips the next instruction if the key stored in VX is pressed.
        EXA1	Skips the next instruction if the key stored in VX isn't pressed.
        FX07	Sets VX to the value of the delay timer.
        FX0A	A key press is awaited, and then stored in VX.
        FX15	Sets the delay timer to VX.
        FX18	Sets the sound timer to VX.
        FX1E	Adds VX to I.[3]
        FX29	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
        FX33	Stores the Binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
        FX55	Stores V0 to VX in memory starting at address I.[4]
        FX65	Fills V0 to VX with values from memory starting at address I.[4]
    */
    
    fn set_register_constant(&mut self, opcode: u16) {
        self.v[self.opcode_digit(opcode, 2)] = (opcode & 0x00FF) as u8;
    }
    
    fn add_to_register(&mut self, opcode: u16) {
        self.v[self.opcode_digit(opcode, 2)] += (opcode & 0x00FF) as u8;
    }
    
    // These should probably accept a function as an argument
    fn skip_if_equals_constant(&mut self, opcode: u16) {
        if self.v[self.opcode_digit(opcode, 2)] == (opcode & 0x00FF) as u8 {
            self.pc += 2;
        }
    }
    
    fn skip_if_not_equals_constant(&mut self, opcode: u16) {
        if self.v[self.opcode_digit(opcode, 2)] != (opcode & 0x00FF) as u8 {
            self.pc += 2;
        }
    }
    
    fn skip_if_equals_v(&mut self, opcode: u16) {
        if self.v[self.opcode_digit(opcode, 2)] == self.v[self.opcode_digit(opcode, 3)] {
            self.pc += 2;
        }
    }
    
    fn skip_if_not_equals_v(&mut self, opcode: u16) {
        if self.v[self.opcode_digit(opcode, 2)] != self.v[self.opcode_digit(opcode, 3)] {
            self.pc += 2;
        }
    }
    
    fn set_register_value(&mut self, opcode: u16) {
        self.v[self.opcode_digit(opcode, 2)] = self.v[self.opcode_digit(opcode, 3)];
    }
    
    fn set_register_or(&mut self, opcode: u16) {
        self.v[self.opcode_digit(opcode, 2)] |= self.v[self.opcode_digit(opcode, 3)];
    }
    
    fn set_register_and(&mut self, opcode: u16) {
        self.v[self.opcode_digit(opcode, 2)] &= self.v[self.opcode_digit(opcode, 3)];
    }
    
    fn set_register_xor(&mut self, opcode: u16) {
        self.v[self.opcode_digit(opcode, 2)] ^= self.v[self.opcode_digit(opcode, 3)];
    }
    
    fn set_i_to_opcode(&mut self, opcode: u16) {
        self.i = opcode & 0x0FFF;
    }
    
    fn call_subroutine(&mut self, opcode: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.jump_to_address(opcode);
    }
    
    fn jump_to_address(&mut self, opcode: u16) {
        self.pc = (opcode & 0x0FFF) - 2; // program counter increments by 2 afterwards, always   
    }
    
    fn return_from_subroutine(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }
    
    fn add(&mut self, opcode: u16) {
        if self.v[self.opcode_digit(opcode, 3)] > (0xFF - self.v[self.opcode_digit(opcode, 2)]) {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }
        self.v[self.opcode_digit(opcode, 2)] = 
            self.v[self.opcode_digit(opcode, 2)].wrapping_add(self.v[self.opcode_digit(opcode, 3)]);
    }
    
    fn opcode_digit(&self, opcode: u16, digit: u8) -> usize {
        match digit {
            1 => ((opcode & 0xF000) >> 12) as usize,
            2 => ((opcode & 0x0F00) >> 8) as usize,
            3 => ((opcode & 0x00F0) >> 4) as usize,
            4 => (opcode & 0x000F) as usize,
            _ => 0 as usize
        }
    }
    
}
