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

pub fn unload_emulator() {
    let mut emulator_instance = EMULATOR_INSTANCE.lock().unwrap();
    *emulator_instance = None;
}

pub fn render_frame() -> Option<Vec<u32>> {
    let mut emulator_instance = EMULATOR_INSTANCE.lock().unwrap();
    
    if let Some(ref mut emulator) = *emulator_instance {
        Some(emulator.render_frame())
    } else {
        None
    }
}

pub fn handle_vblank() {
    let mut emulator_instance = EMULATOR_INSTANCE.lock().unwrap();
    
    if let Some(ref mut emulator) = *emulator_instance {
        emulator.handle_vblank();
    }
}

pub fn set_buttons_state(button_states: Vec<u8>) {
    let mut emulator_instance = EMULATOR_INSTANCE.lock().unwrap();
    
    if let Some(ref mut emulator) = *emulator_instance {
        if button_states.len() == 8 {
            emulator.set_button_states(
                button_states[0], // Up
                button_states[1], // Down
                button_states[2], // Left
                button_states[3], // Right
                button_states[4], // A
                button_states[5], // B
                button_states[6], // Start
                button_states[7], // Select
            );
        }
    }
}

#[frb]
pub fn load(rom_data: Vec<u8>) {
    load_rom(rom_data);
}

#[frb]
pub fn unload() {
    unload_emulator();
}

#[frb]
pub fn render() -> Option<Vec<u32>> {
    render_frame()
}

#[frb]
pub fn vblank() {
    handle_vblank();
}

#[frb]
pub fn set_buttons(button_states: Vec<u8>) {
    set_buttons_state(button_states);
}
