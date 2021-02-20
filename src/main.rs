use std::fmt;

extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const RAM_SIZE: usize = 4096;
const REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const PIXEL_SIZE: usize = 8;
const DISPLAY_PIXELS: usize = DISPLAY_WIDTH * PIXEL_SIZE * DISPLAY_HEIGHT * PIXEL_SIZE;

struct Device {
    ram: [u8; RAM_SIZE],
    stack: [u16; STACK_SIZE],
    registers: [u8; REGISTERS],
    display: [u32; DISPLAY_PIXELS],
    reg_i: u8,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
    window: minifb::Window,
}

impl Device {
    fn new() -> Self {
        Device {
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            registers: [0; REGISTERS],
            display: [0; DISPLAY_PIXELS],
            reg_i: 0,
            delay: 0,
            sound: 0,
            pc: 0,
            sp: 0,
            window: minifb::Window::new(
                "Chip8",
                DISPLAY_WIDTH * PIXEL_SIZE,
                DISPLAY_HEIGHT * PIXEL_SIZE,
                WindowOptions::default(),
            )
            .unwrap(),
        }
    }

    fn step(&mut self) {
        self.window
            .update_with_buffer(
                &self.display,
                DISPLAY_WIDTH * PIXEL_SIZE,
                DISPLAY_HEIGHT * PIXEL_SIZE,
            )
            .unwrap();
    }
}

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut device = Device::new();

    loop {
        device.step();
    }

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
