use std::fmt;

extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const RAM_SIZE: usize = 4096;
const REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const PIXEL_SIZE: usize = 8;

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
        }
    }

    fn step(&mut self) -> bool {
        self.display.refresh();

        self.display.is_open()
    }
}

fn main() {
    let mut device = Device::new();

    while device.step() {}

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
