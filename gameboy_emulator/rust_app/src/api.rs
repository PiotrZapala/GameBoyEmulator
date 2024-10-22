use lazy_static::lazy_static;
use std::sync::Mutex;

use flutter_rust_bridge::frb;

use crate::cartridge::CARTRIDGE;
use crate::emulator::EMULATOR;

lazy_static! {
    static ref EMULATOR_INSTANCE: Mutex<Option<EMULATOR>> = Mutex::new(None);
}

pub fn load_rom(rom_data: Vec<u8>) {
    let cartridge = CARTRIDGE::new(rom_data);
    let emulator = EMULATOR::new(cartridge);
    
    let mut emulator_instance = EMULATOR_INSTANCE.lock().unwrap();
    *emulator_instance = Some(emulator);
}

pub fn render_frame() -> Option<Vec<u32>> {
    let mut emulator_instance = EMULATOR_INSTANCE.lock().unwrap();
    
    if let Some(ref mut emulator) = *emulator_instance {
        Some(emulator.render_frame())
    } else {
        None
    }
}

#[frb]
pub fn load(rom_data: Vec<u8>) {
    load_rom(rom_data);
}

#[frb]
pub fn render() -> Option<Vec<u32>> {
    render_frame()
}
