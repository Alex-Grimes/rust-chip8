extern crate rand;
extern crate sdl2;
mod font;
mod modules;
mod processor;

use std::env;
use std::thread;
use std::time::Duration;

use modules::{CartridgeModule, DisplayModule, InputModule, SoundModule};

use processor::Processor;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_MEMORY: usize = 4096;

fn main() {
    let sleep_duration = Duration::from_millis(2);
    let sdl_context = sdl2::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];
    let cartridge_driver = CartridgeModule::new(cartridge_filename);
    let mut display_driver = DisplayModule::new(&sdl_context);
    let mut input_driver = InputModule::new(&sdl_context);
    let sound_driver = SoundModule::new(&sdl_context);
    let mut processor = Processor::new();

    processor.load(&cartridge_driver.rom);

    while let Ok(keypad) = input_driver.poll() {
        let output = processor.tick(keypad);

        if output.vram_changed {
            display_driver.draw(output.vram);
        }

        if output.beep {
            sound_driver.start_beep();
        } else {
            sound_driver.stop_beep();
        }

        thread::sleep(sleep_duration);
    }
}
