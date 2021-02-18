use std::fmt;

extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const RAM_SIZE: usize = 4096;
const REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;

struct Device {
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

    fn timer_tick(&mut self) {
        self.delay = self.delay.saturating_sub(1);
        self.sound = self.sound.saturating_sub(1);
    }

    fn timers_active(&self) -> bool {
        self.delay > 0 || self.sound > 0
    }
}

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600 * 2)));

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    // window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    // }
}
