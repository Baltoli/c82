use std::fmt;

extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const RAM_SIZE: usize = 4096;
const REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const PIXEL_SIZE: usize = 8;

#[derive(Debug)]
enum ArithOpcode {
    Load(),
    Or(),
    And(),
    XOr(),
    Add(),
    Sub(),
    Shr(),
    SubN(),
    Shl(),
}

#[derive(Debug)]
enum Instruction {
    Sys(u16),
    Cls(),
    Ret(),
    Jump(u16),
    Call(u16),
    SkipEqIm(u8, u8),
    SkipNotEqIm(u8, u8),
    SkipEq(u8, u8),
    SkipNotEq(u8, u8),
    LoadIm(u8, u8),
    AddIm(u8, u8),
    Op(ArithOpcode, u8, u8),
    LoadI(u16),
    JumpV0(u16),
    Rand(u8, u8),
    Draw(u8, u8, u8),
    SkipKey(u8),
    SkipNotKey(u8),
    GetDelay(u8),
    WaitKey(u8),
    SetDelay(u8),
    SetSound(u8),
    AddI(u8),
    DigitSprite(u8),
    BCD(u8),
    StoreMem(u8),
    LoadMem(u8),
}

impl Instruction {
    fn parse(code: u16) -> Self {
        todo!()
    }
}

struct Display {
    width: usize,
    height: usize,
    scale: usize,
    buffer: Vec<u32>,
    window: Window,
}

impl Display {
    fn new(width: usize, height: usize, scale: usize) -> Self {
        Display {
            width,
            height,
            scale,
            buffer: vec![0; width * height * scale * scale],
            window: Window::new(
                "Chip8",
                width * scale,
                height * scale,
                WindowOptions::default(),
            )
            .unwrap(),
        }
    }

    fn refresh(&mut self) {
        self.window
            .update_with_buffer(
                &self.buffer,
                self.width * self.scale,
                self.height * self.scale,
            )
            .unwrap();
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

struct Device {
    display: Display,
    ram: [u8; RAM_SIZE],
    stack: [u16; STACK_SIZE],
    registers: [u8; REGISTERS],
    reg_i: u8,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
    program: Vec<Instruction>,
}

impl Device {
    fn new() -> Self {
        Device {
            display: Display::new(DISPLAY_WIDTH, DISPLAY_HEIGHT, PIXEL_SIZE),
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            registers: [0; REGISTERS],
            reg_i: 0,
            delay: 0,
            sound: 0,
            pc: 0,
            sp: 0,
            program: Vec::new(),
        }
    }

    fn step(&mut self) -> bool {
        self.display.refresh();

        self.display.is_open()
    }

    fn register(&mut self, idx: u8) -> &u8 {
        &self.registers[idx as usize]
    }

    fn execute(&mut self, inst: Instruction) {
        use Instruction::*;

        match inst {
            Cls() => todo!(),
            Ret() => todo!(),
            Jump(addr) => self.pc = addr,
            Call(addr) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = addr;
            }
            SkipEqIm(reg, val) => {
                if *self.register(reg) == val {
                    self.pc += 2;
                }
            }
            SkipNotEqIm(reg, val) => {
                if *self.register(reg) != val {
                    self.pc += 2;
                }
            }
            SkipEq(reg_a, reg_b) => {
                if *self.register(reg_a) != *self.register(reg_b) {
                    self.pc += 2;
                }
            }
            // SkipNotEq(u8, u8),
            // LoadIm(u8, u8),
            // AddIm(u8, u8),
            // Op(ArithOpcode, u8, u8),
            // LoadI(u16),
            // JumpV0(u16),
            // Rand(u8, u8),
            // Draw(u8, u8, u8),
            // SkipKey(u8),
            // SkipNotKey(u8),
            // GetDelay(u8),
            // WaitKey(u8),
            // SetDelay(u8),
            // SetSound(u8),
            // AddI(u8),
            // DigitSprite(u8),
            // BCD(u8),
            // StoreMem(u8),
            // LoadMem(u8),
            _ => todo!(),
        }
    }
}

fn main() {
    let mut device = Device::new();

    device.execute(Instruction::Jump(12));
    println!("{}\n", device.pc);

    // while device.step() {}

    // let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // let mut window = Window::new(
    //     "Test - ESC to exit",
    //     WIDTH,
    //     HEIGHT,
    //     WindowOptions::default(),
    // )

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600 * 2)));

    // // while window.is_open() && !window.is_key_down(Key::Escape) {
    // // window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    // // }
}
