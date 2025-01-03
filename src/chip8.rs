use std::{io::Read, sync::{Arc, Mutex}};

use rand::{rngs::{StdRng, ThreadRng}, Rng, SeedableRng};

pub const START_ADDRESS: usize = 0x200;
pub const FONTSET_SIZE: usize = 80;
pub const FONTSET_START_ADDRESS: usize = 0x50;
pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;

pub const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [u8; 16],
    pub display: [u32; 64 * 32],
    pub opcode: u16,

    pub table: [Option<fn(&mut Chip8)>; 0xF + 1],
    pub table0: [Option<fn(&mut Chip8)>; 0xE + 1],
    pub table8: [Option<fn(&mut Chip8)>; 0xE + 1],
    pub tableE: [Option<fn(&mut Chip8)>; 0xE + 1],
    pub tableF: [Option<fn(&mut Chip8)>; 0x65 + 1],

    pub rand_gen: StdRng,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS as u16,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            display: [0; 64 * 32],
            opcode: 0,
            table: [None; 0xF + 1],
            table0: [None; 0xE + 1],
            table8: [None; 0xE + 1],
            tableE: [None; 0xE + 1],
            tableF: [None; 0x65 + 1],

            rand_gen: StdRng::from_entropy(),
        };

        chip8.pc = START_ADDRESS as u16;

        // Load fontset
        for i in 0..FONTSET_SIZE {
            chip8.memory[FONTSET_START_ADDRESS + i] = FONTSET[i];
        }

        // Setup RNG
        let rand_byte = rand::distributions::Uniform::new_inclusive(0_u8, 255_u8);

        // Set up function pointer table
        chip8.table[0x0] = Some(Chip8::table0);
        chip8.table[0x1] = Some(Chip8::OP_1NNN);
        chip8.table[0x2] = Some(Chip8::OP_2NNN);
        chip8.table[0x3] = Some(Chip8::OP_3XKK);
        chip8.table[0x4] = Some(Chip8::OP_4XKK);
        chip8.table[0x5] = Some(Chip8::OP_5XY0);
        chip8.table[0x6] = Some(Chip8::OP_6XKK);
        chip8.table[0x7] = Some(Chip8::OP_7XKK);
        chip8.table[0x8] = Some(Chip8::table8);
        chip8.table[0x9] = Some(Chip8::OP_9XY0);
        chip8.table[0xA] = Some(Chip8::OP_ANNN);
        chip8.table[0xB] = Some(Chip8::OP_BNNN);
        chip8.table[0xC] = Some(Chip8::OP_CXKK);
        chip8.table[0xD] = Some(Chip8::OP_DXYN);
        chip8.table[0xE] = Some(Chip8::tableE);
        chip8.table[0xF] = Some(Chip8::tableF);

        for i in 0..=0xE {
            chip8.table0[i] = Some(Chip8::op_null);
            chip8.table8[i] = Some(Chip8::op_null);
            chip8.tableE[i] = Some(Chip8::op_null);
        }

        chip8.table0[0x0] = Some(Chip8::OP_00E0);
        chip8.table0[0xE] = Some(Chip8::OP_00EE);

        chip8.table8[0x0] = Some(Chip8::OP_8XY0);
        chip8.table8[0x1] = Some(Chip8::OP_8XY1);
        chip8.table8[0x2] = Some(Chip8::OP_8XY2);
        chip8.table8[0x3] = Some(Chip8::OP_8XY3);
        chip8.table8[0x4] = Some(Chip8::OP_8XY4);
        chip8.table8[0x5] = Some(Chip8::OP_8XY5);
        chip8.table8[0x6] = Some(Chip8::OP_8XY6);
        chip8.table8[0x7] = Some(Chip8::OP_8XY7);
        chip8.table8[0xE] = Some(Chip8::OP_8XYE);

        chip8.tableE[0x1] = Some(Chip8::OP_EXA1);
        chip8.tableE[0xE] = Some(Chip8::OP_EX9E);

        for i in 0..=0x65 {
            chip8.tableF[i] = Some(Chip8::op_null);
        }

        chip8.tableF[0x07] = Some(Chip8::OP_FX07);
        chip8.tableF[0x0A] = Some(Chip8::OP_FX0A);
        chip8.tableF[0x15] = Some(Chip8::OP_FX15);
        chip8.tableF[0x18] = Some(Chip8::OP_FX18);
        chip8.tableF[0x1E] = Some(Chip8::OP_FX1E);
        chip8.tableF[0x29] = Some(Chip8::OP_FX29);
        chip8.tableF[0x33] = Some(Chip8::OP_FX33);
        chip8.tableF[0x55] = Some(Chip8::OP_FX55);
        chip8.tableF[0x65] = Some(Chip8::OP_FX65);

        chip8
    }

    fn table0(&mut self) {
        if let Some(func) = self.table0[(self.opcode & 0x000F) as usize] {
            func(self);
        }
    }

    fn table8(&mut self) {
        if let Some(func) = self.table8[(self.opcode & 0x000F) as usize] {
            func(self);
        }
    }

    fn tableE(&mut self) {
        if let Some(func) = self.tableE[(self.opcode & 0x000F) as usize] {
            func(self);
        }
    }

    fn tableF(&mut self) {
        if let Some(func) = self.tableF[(self.opcode & 0x00FF) as usize] {
            func(self);
        }
    }

    fn op_null(&mut self) {}

    pub fn load_rom(&mut self, filename: &str) {
        println!("Attempting to load ROM: {}", filename);
        let file = std::fs::File::open(filename);

        if let Ok(mut file) = file {
            println!("ROM file opened successfully.");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .expect("Failed to read ROM file");

            // Debug: Print ROM size and contents
            println!("ROM size: {} bytes", buffer.len());
            for (i, byte) in buffer.iter().enumerate() {
                println!("Memory[{:X}] = {:02X}", 0x200 + i, byte);
                self.memory[0x200 + i] = *byte; // Load the byte into memory
            }
        } else {
            println!("Failed to open ROM file: {}", filename);
        }
    }

    /// Clear the display
    pub fn OP_00E0(&mut self) {
        self.display = [0; 64 * 32];
    }

    /// Return from a subroutine
    pub fn OP_00EE(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    /// Jump to location nnn
    pub fn OP_1NNN(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.pc = address;
    }

    /// Call subroutine at nnn
    pub fn OP_2NNN(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    pub fn OP_3XKK(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] == byte {
            self.pc += 2;
        }
    }

    pub fn OP_4XKK(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] != byte {
            self.pc += 2;
        }
    }

    pub fn OP_5XY0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    pub fn OP_6XKK(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = byte;
    }

    pub fn OP_7XKK(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(byte);
    }

    pub fn OP_8XY0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] = self.registers[vy as usize];
    }

    pub fn OP_8XY1(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] |= self.registers[vy as usize];
    }

    pub fn OP_8XY2(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] &= self.registers[vy as usize];
    }

    pub fn OP_8XY3(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] ^= self.registers[vy as usize];
    }

    pub fn OP_8XY4(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        let sum = self.registers[vx as usize] as u16 + self.registers[vy as usize] as u16;
        self.registers[0xF] = if sum > 255 { 1 } else { 0 };
        self.registers[vx as usize] = sum as u8;
    }

    pub fn OP_8XY5(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[0xF] = if self.registers[vx as usize] > self.registers[vy as usize] {
            1
        } else {
            0
        };
        self.registers[vx as usize] =
            self.registers[vx as usize].wrapping_sub(self.registers[vy as usize]);
    }

    pub fn OP_8XY6(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[0xF] = self.registers[vx as usize] & 0x1;
        self.registers[vx as usize] >>= 1;
    }

    pub fn OP_8XY7(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[0xF] = if self.registers[vy as usize] > self.registers[vx as usize] {
            1
        } else {
            0
        };
        self.registers[vx as usize] =
            self.registers[vy as usize].wrapping_sub(self.registers[vx as usize]);
    }

    pub fn OP_8XYE(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[0xF] = (self.registers[vx as usize] & 0x80) >> 7;
        self.registers[vx as usize] <<= 1;
    }

    pub fn OP_9XY0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    pub fn OP_ANNN(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.index = address;
    }

    pub fn OP_BNNN(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.pc = address + self.registers[0] as u16;
    }

    pub fn OP_CXKK(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        let rng: u8 = self.rand_gen.gen();
        self.registers[vx as usize] = rng & byte;
    }

    pub fn OP_DXYN(&mut self) {
        let vx = ((self.opcode & 0x0F00u16) >> 8) as u8;
        let vy = ((self.opcode & 0x00F0u16) >> 4) as u8;
        let height = (self.opcode & 0x000Fu16) as u8;
    
        let x_pos = self.registers[vx as usize] % VIDEO_WIDTH as u8;
        let y_pos = self.registers[vy as usize] % VIDEO_HEIGHT as u8;
    
        self.registers[0xF] = 0;
    
        for row in 0..height {
            let sprite_byte = self.memory[self.index as usize + row as usize];
    
            for col in 0..8 { // We have 8 columns to deal with (not just 7)
                let sprite_pixel = sprite_byte & (0x80 >> col);
                let display_x = (x_pos as usize + col as usize) % VIDEO_WIDTH; // Wrap around to the other side of the screen
                let display_y = (y_pos as usize + row as usize) % VIDEO_HEIGHT; // Same for vertical
    
                let screen_pixel = &mut self.display[display_y * VIDEO_WIDTH + display_x];
    
                if sprite_pixel != 0 {
                    if *screen_pixel == 0xFFFFFFFF {
                        self.registers[0xF] = 1;
                    }
    
                    *screen_pixel ^= 0xFFFFFFFF; // XOR to flip the pixel
                }
            }
        }
    }
    

    pub fn OP_EX9E(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let key: u8 = self.registers[vx as usize];

        if self.keypad[key as usize] != 0 {
            self.pc += 2;
        }
    }

    pub fn OP_EXA1(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let key: u8 = self.registers[vx as usize];

        if self.keypad[key as usize] == 0 {
            self.pc += 2;
        }
    }

    pub fn OP_FX07(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        self.registers[vx as usize] = self.delay_timer;
    }

    pub fn OP_FX0A(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as u8;
        
        if self.keypad[0] != 0 {
            self.registers[vx as usize] = 0;
        } else if self.keypad[1] != 0 {
            self.registers[vx as usize] = 1;
        } else if self.keypad[2] != 0 {
            self.registers[vx as usize] = 2;
        } else if self.keypad[3] != 0 {
            self.registers[vx as usize] = 3;
        } else if self.keypad[4] != 0 {
            self.registers[vx as usize] = 4;
        } else if self.keypad[5] != 0 {
            self.registers[vx as usize] = 5;
        } else if self.keypad[6] != 0 {
            self.registers[vx as usize] = 6;
        } else if self.keypad[7] != 0 {
            self.registers[vx as usize] = 7;
        } else if self.keypad[8] != 0 {
            self.registers[vx as usize] = 8;
        } else if self.keypad[9] != 0 {
            self.registers[vx as usize] = 9;
        } else if self.keypad[10] != 0 {
            self.registers[vx as usize] = 10;
        } else if self.keypad[11] != 0 {
            self.registers[vx as usize] = 11;
        } else if self.keypad[12] != 0 {
            self.registers[vx as usize] = 12;
        } else if self.keypad[13] != 0 {
            self.registers[vx as usize] = 13;
        } else if self.keypad[14] != 0 {
            self.registers[vx as usize] = 14;
        } else if self.keypad[15] != 0 {
            self.registers[vx as usize] = 15;
        } else {
            self.pc -= 2;
        }
    }

    pub fn OP_FX15(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.delay_timer = self.registers[vx as usize];
    }

    pub fn OP_FX18(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.sound_timer = self.registers[vx as usize];
    }

    pub fn OP_FX1E(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.index += self.registers[vx as usize] as u16;
    }

    pub fn OP_FX29(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let digit: u8 = self.registers[vx as usize];
        self.index = FONTSET_START_ADDRESS as u16 + (5 * digit as u16);
    }

    pub fn OP_FX33(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let mut value = self.registers[vx as usize];

        self.memory[self.index as usize + 2] = value % 10;
        value /= 10;
        self.memory[self.index as usize + 1] = value % 10;
        value /= 10;
        self.memory[self.index as usize] = value % 10;
    }

    pub fn OP_FX55(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        for i in 0..=vx {
            self.memory[self.index as usize + i as usize] = self.registers[i as usize];
        }
    }

    pub fn OP_FX65(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        for i in 0..=vx {
            self.registers[i as usize] = self.memory[self.index as usize + i as usize];
        }
    }

    pub fn key(&mut self, key: u8, state: bool) {
        self.keypad[key as usize] = if state { 1 } else { 0 };
    }

    pub fn cycle(&mut self) {
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8) | self.memory[(self.pc + 1) as usize] as u16;

        self.pc += 2;

        if let Some(func) = self.table[((self.opcode & 0xF000) >> 12) as usize] {
            func(self);
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
