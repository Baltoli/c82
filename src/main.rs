use eframe::{egui, epi};

// use std::fmt;

// const RAM_SIZE: usize = 4096;
// const REGISTERS: usize = 16;
// const STACK_SIZE: usize = 16;

// struct Device {
//     ram: [u8; RAM_SIZE],
//     stack: [u16; STACK_SIZE],
//     registers: [u8; REGISTERS],
//     reg_i: u8,
//     delay: u8,
//     sound: u8,
//     pc: u16,
//     sp: u8,
// }

// impl Device {
//     fn new() -> Self {
//         Device {
//             ram: [0; RAM_SIZE],
//             stack: [0; STACK_SIZE],
//             registers: [0; REGISTERS],
//             reg_i: 0,
//             delay: 0,
//             sound: 0,
//             pc: 0,
//             sp: 0,
//         }
//     }
// }

// impl fmt::Display for Device {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         write!(f, "(device)")
//     }
// }

struct DeviceApp {
    label: String,
}

impl epi::App for DeviceApp {
    fn name(&self) -> &str {
        "Chip8"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });
        });
    }
}

fn main() {
    let app = DeviceApp {
        label: "place".into(),
    };
    eframe::run_native(Box::new(app));
}
