use eframe::{egui, epi};

use std::fmt;

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

struct DeviceViewer {
    device: Device,
}

impl DeviceViewer {
    fn ram_width() -> usize {
        16
    }

    fn ram_height() -> usize {
        RAM_SIZE / Self::ram_width()
    }
}

impl epi::App for DeviceViewer {
    fn name(&self) -> &str {
        "Chip8"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        egui::Window::new("Memory").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.monospace("        ");
                for x in 0..Self::ram_width() {
                    ui.monospace(format!("{:0>2X}", x));
                }
            });

            egui::ScrollArea::auto_sized().show(ui, |ui| {
                for y in 0..Self::ram_height() {
                    ui.horizontal(|ui| {
                        ui.monospace(format!("{:0>8X}", y * Self::ram_width()));

                        for x in 0..Self::ram_width() {
                            ui.monospace(format!(
                                "{:0>2X}",
                                self.device.ram[y * Self::ram_width() + x]
                            ));
                        }
                    });
                }
            });
        });

        self.device.timer_tick();

        if self.device.timers_active() {
            ctx.request_repaint();
        }
    }
}

fn main() {
    let app = DeviceViewer {
        device: Device::new(),
    };

    eframe::run_native(Box::new(app));
}
