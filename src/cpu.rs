const FONTS: [u8; 80] = [
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

pub struct Cpu {
  pub memory: [u8; 4096],
  pub v: [u8; 16],        // V0-VF registers
  pub i: u16,             // Index register
  pub pc: u16,            // Program Counter
  pub stack: [u16; 16],
  pub sp: u8,             // Stack pointer
  pub delay_timer: u8,
  pub sound_timer: u8,
  pub keypad: [bool; 16],
  pub display: [bool; 64 * 32],
}

impl Cpu {
  pub fn new() -> Self {
    let mut memory= [0u8; 4096];
    memory[0x050..0x0A0].copy_from_slice(&FONTS);

    Cpu {
      memory,
      v: [0; 16],
      i: 0,
      pc: 0x200,          // Programs start at 0x200
      stack: [0; 16],
      sp: 0,
      delay_timer: 0,
      sound_timer: 0,
      keypad: [false; 16],
      display: [false; 64 * 32],
    }
  }

  pub fn fetch(&mut self) -> u16 {
    let opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
    self.pc += 2;
    opcode
  }

  pub fn execute(&mut self, opcode: u16) {
    let nibble = (opcode & 0xF000) >> 12;
    let x = ((opcode & 0x0F00) >> 8) as usize;
    let y = ((opcode & 0x00F0) >> 4) as usize;
    let n = (opcode & 0x000F) as u8;
    let nn = (opcode & 0x00FF) as u8;
    let nnn = (opcode & 0x0FFF) as u16;

    match nibble {
      0x0 => self.display = [false; 64 * 32],
      0x1 => self.pc = nnn,
      0x6 => self.v[x] = nn,
      0x7 => self.v[x] = self.v[x].wrapping_add(nn),
      0xA => self.i = nnn,
      0xD => {
        let x_pos = self.v[x] as usize % 64;
        let y_pos = self.v[y] as usize % 32;
        self.v[0xF] = 0;

        for row in 0..n {
          let sprite_byte = self.memory[self.i as usize + row as usize];

          for col in 0..8 {
            let bit = (sprite_byte >> (7 - col)) & 0x1;

            // The following 3 lines are to calculate the index of the flattened display array
            let x_wrapped = (x_pos + col) % 64;
            let y_wrapped = (y_pos + row as usize) % 32;
            let idx = y_wrapped * 64 + x_wrapped;

            if bit == 1 {
              if self.display[idx] {
                self.v[0xF] = 1;
              }
              self.display[idx] ^= true;
            }
          }
        }
      }
      _ => eprintln!("Unknown opcode: {:#06X}", opcode),
    }
  }
}
