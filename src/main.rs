use std::{env, thread};
use std::io::{Read};
use std::process;
use std::time::{Duration, Instant};

use chip8emu::chip8::*;

extern crate sdl2;
use chip8emu::platform::Platform;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::{
    SDL_Event, SDL_SetWindowPosition,
};


fn main() {
    println!("CHIP-8 Emulator Starting...");
    let args: Vec<String> = env::args().collect();  

    if args.len() != 4 {
        eprintln!("Usage: {} <Scale> <Delay> <ROM>", args[0]);
        process::exit(1);
    }

    let video_scale = args[1].parse::<u16>().expect("Failed to parse Scale");
    let cycle_delay = 4; // sets to 250Hz
    let rom_filename = &args[3];

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

    let mut chip8 = Chip8::new();
    println!("Loading ROM: {}", rom_filename);
    chip8.load_rom(rom_filename);
    println!("ROM loaded");

    let mut quit = false;
    let video_pitch = std::mem::size_of::<u32>() * VIDEO_WIDTH;
    let mut last_cycle_time = Instant::now();

    loop {
        let mut quit = false;
        let mut event = SDL_Event { type_: 0 };
    
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => chip8.key(0x1, false),
                        Keycode::Num2 => chip8.key(0x2, false),
                        Keycode::Num3 => chip8.key(0x3, false),
                        Keycode::Num4 => chip8.key(0xC, false),
                        Keycode::Q => chip8.key(0x4, false),
                        Keycode::W => chip8.key(0x5, false),
                        Keycode::E => chip8.key(0x6, false),
                        Keycode::R => chip8.key(0xD, false),
                        Keycode::A => chip8.key(0x7, false),
                        Keycode::S => chip8.key(0x8, false),
                        Keycode::D => chip8.key(0x9, false),
                        Keycode::F => chip8.key(0xE, false),
                        Keycode::Z => chip8.key(0xA, false),
                        Keycode::X => chip8.key(0x0, false),
                        Keycode::C => chip8.key(0xB, false),
                        Keycode::V => chip8.key(0xF, false),
                        _ => (),
                    }
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => chip8.key(0x1, true),
                        Keycode::Num2 => chip8.key(0x2, true),
                        Keycode::Num3 => chip8.key(0x3, true),
                        Keycode::Num4 => chip8.key(0xC, true),
                        Keycode::Q => chip8.key(0x4, true),
                        Keycode::W => chip8.key(0x5, true),
                        Keycode::E => chip8.key(0x6, true),
                        Keycode::R => chip8.key(0xD, true),
                        Keycode::A => chip8.key(0x7, true),
                        Keycode::S => chip8.key(0x8, true),
                        Keycode::D => chip8.key(0x9, true),
                        Keycode::F => chip8.key(0xE, true),
                        Keycode::Z => chip8.key(0xA, true),
                        Keycode::X => chip8.key(0x0, true),
                        Keycode::C => chip8.key(0xB, true),
                        Keycode::V => chip8.key(0xF, true),
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        let current_time = Instant::now();
        let dt = (current_time - last_cycle_time).as_millis() as f32;

        if dt >= cycle_delay as f32 {
            chip8.cycle(); // Run a single cycle of the emulator

            platform.update(
                chip8.display.as_ptr() as *const std::ffi::c_void,
                video_pitch.try_into().unwrap(),
            );

            last_cycle_time = Instant::now(); // Reset the cycle time
        } else {
            // Sleep to avoid overloading the CPU if cycles are executing too quickly
            let remaining_time = cycle_delay as f32 - dt;
            thread::sleep(Duration::from_millis(remaining_time as u64));
        }
    }

    println!("Exiting CHIP-8 Emulator.");
}