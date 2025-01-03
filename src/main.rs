use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use chip8emu::chip8::*;
use chip8emu::platform::Platform;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::{SDL_Event, SDL_SetWindowPosition};

use eframe::egui;

fn main() {
    println!("CHIP-8 Emulator Starting...");
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <Scale> <Delay> <ROM>", args[0]);
        std::process::exit(1);
    }

    let video_scale = args[1].parse::<u16>().expect("Failed to parse Scale");
    let cycle_delay = 2; // 500Hz
    let rom_filename = &args[3];

    // Shared CHIP-8 state
    let chip8 = Arc::new(Mutex::new(Chip8::new()));
    chip8.lock().unwrap().load_rom(rom_filename);
    println!("ROM loaded");

    // Start Emulator in a Secondary Thread
    let chip8_for_emulator = chip8.clone();
    thread::spawn(move || {
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut platform = Platform::new(
            "CHIP-8 Emulator",
            (VIDEO_WIDTH * video_scale as usize) as i32,
            (VIDEO_HEIGHT * video_scale as usize) as i32,
            VIDEO_WIDTH as i32,
            VIDEO_HEIGHT as i32,
        );

        unsafe {
            SDL_SetWindowPosition(
                platform.window,
                sdl2::sys::SDL_WINDOWPOS_CENTERED_MASK as i32,
                sdl2::sys::SDL_WINDOWPOS_CENTERED_MASK as i32,
            );
        }

        let video_pitch = std::mem::size_of::<u32>() * VIDEO_WIDTH;
        let mut last_cycle_time = Instant::now();

        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        let mut chip8 = chip8_for_emulator.lock().unwrap();
                        chip8.key(get_key_mapping(keycode), true);
                    }
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        let mut chip8 = chip8_for_emulator.lock().unwrap();
                        chip8.key(get_key_mapping(keycode), false);
                    }
                    _ => {}
                }
            }

            let current_time = Instant::now();
            let dt = (current_time - last_cycle_time).as_millis() as f32;

            if dt >= cycle_delay as f32 {
                let mut chip8 = chip8_for_emulator.lock().unwrap();
                chip8.cycle();
                platform.update(
                    chip8.display.as_ptr() as *const std::ffi::c_void,
                    video_pitch.try_into().unwrap(),
                );

                last_cycle_time = Instant::now();
            } else {
                thread::sleep(Duration::from_millis((cycle_delay as f32 - dt) as u64));
            }
        }
    });

    // Run Debug Stats GUI on Main Thread
    let chip8_for_gui = chip8.clone();
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "CHIP-8 Debug Stats",
        options,
        Box::new(|_cc| Ok(Box::new(DebugApp::new(chip8_for_gui)))),
    );
}

// Key mapping for the CHIP-8 keyboard
fn get_key_mapping(keycode: Keycode) -> u8 {
    match keycode {
        Keycode::Num1 => 0x1,
        Keycode::Num2 => 0x2,
        Keycode::Num3 => 0x3,
        Keycode::Num4 => 0xC,
        Keycode::Q => 0x4,
        Keycode::W => 0x5,
        Keycode::E => 0x6,
        Keycode::R => 0xD,
        Keycode::A => 0x7,
        Keycode::S => 0x8,
        Keycode::D => 0x9,
        Keycode::F => 0xE,
        Keycode::Z => 0xA,
        Keycode::X => 0x0,
        Keycode::C => 0xB,
        Keycode::V => 0xF,
        _ => 0xFF,
    }
}

// Debug Stats GUI
struct DebugApp {
    chip8: Arc<Mutex<Chip8>>,
}

impl DebugApp {
    fn new(chip8: Arc<Mutex<Chip8>>) -> Self {
        Self { chip8 }
    }
}

impl eframe::App for DebugApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let chip8 = self.chip8.lock().unwrap();
            ui.heading("CHIP-8 Debug Stats");
            ui.label(format!("PC: {:04X}", chip8.pc));
            ui.label(format!("Index: {:04X}", chip8.index));
            ui.label(format!("SP: {}", chip8.sp));
            for (idx, reg) in chip8.registers.iter().enumerate() {
                ui.label(format!("V{:X}: {:02X}", idx, reg));
            }
            ui.label(format!("Delay Timer: {}", chip8.delay_timer));
            ui.label(format!("Sound Timer: {}", chip8.sound_timer));
        });

        ctx.request_repaint();
    }
}
