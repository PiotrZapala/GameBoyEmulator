use std::sync::{Arc, Mutex};
use crate::cpu::CPU;

pub struct JOYPAD {
    buttons: u8,             
    select_direction_keys: bool,
    select_action_keys: bool,    
    previous_buttons: u8,       
    cpu: Option<Arc<Mutex<CPU>>>,
}

impl JOYPAD {
    pub fn new() -> Self {
        JOYPAD {
            buttons: 0xFF,              
            select_direction_keys: true, 
            select_action_keys: true,    
            previous_buttons: 0xFF,      
            cpu: None,                  
        }
    }

    pub fn set_cpu(&mut self, cpu: Arc<Mutex<CPU>>) {
        self.cpu = Some(cpu);
    }

    pub fn read_byte(&self) -> u8 {
        let mut result = 0xFF;

        if !self.select_direction_keys {
            result &= self.buttons | 0xF0;
            result &= 0b11101111;
        }

        if !self.select_action_keys {
            result &= (self.buttons >> 4) | 0xF0;
            result &= 0b11011111;
        }

        result
    }

    pub fn write_byte(&mut self, value: u8) {
        self.select_direction_keys = (value & 0x20) == 0;
        self.select_action_keys = (value & 0x10) == 0;
    }

    pub fn set_button_state(&mut self, button: u8, pressed: bool) {
        if pressed {
            self.buttons &= !button;
        } else {
            self.buttons |= button;
        }
    }

    pub fn joypad_state_has_changed(&mut self) -> bool {
        let changed = self.buttons != self.previous_buttons;
        if changed {
            self.previous_buttons = self.buttons;
        }
        changed
    }

    pub fn is_any_group_active(&self) -> bool {
        !self.select_direction_keys || !self.select_action_keys
    }

    pub fn check_for_interrupt(&mut self) {
        if self.joypad_state_has_changed() && self.is_any_group_active() {
            if let Some(ref cpu) = self.cpu {
                let mut cpu_locked = cpu.lock().unwrap();
                cpu_locked.request_interrupt(0b00010000);
            }
        }
    }
}