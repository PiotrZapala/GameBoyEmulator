use std::rc::Rc;
use std::cell::RefCell;

use crate::cpu::CPU;

pub struct JOYPAD {
    joy: u8,  // Joypad register (0xFF00), holding the button state and selected button group
    cpu: Option<Rc<RefCell<CPU>>>,
}

impl JOYPAD {
    pub fn new() -> Self {
        JOYPAD {
            joy: 0xFF,
            cpu: None,
        }
    }

    pub fn set_cpu(&mut self, cpu: Rc<RefCell<CPU>>) {
        self.cpu = Some(cpu);
    }

    pub fn read_byte(&self) -> u8 {
        self.joy
    }

    pub fn write_byte(&mut self, value: u8) {
        self.joy = (self.joy & 0x0F) | (value & 0xF0);
    }

    pub fn press_button(&mut self, button: &str) {
        match button {
            "A" => self.handle_press(0, 0x10),
            "B" => self.handle_press(1, 0x10),
            "Select" => self.handle_press(2, 0x10),
            "Start" => self.handle_press(3, 0x10),
            "Right" => self.handle_press(0, 0x20),
            "Left" => self.handle_press(1, 0x20),
            "Up" => self.handle_press(2, 0x20),
            "Down" => self.handle_press(3, 0x20),
            _ => (),
        }
    }

    pub fn release_button(&mut self, button: &str) {
        match button {
            "A" => self.handle_release(0, 0x10),
            "B" => self.handle_release(1, 0x10),
            "Select" => self.handle_release(2, 0x10),
            "Start" => self.handle_release(3, 0x10),
            "Right" => self.handle_release(0, 0x20),
            "Left" => self.handle_release(1, 0x20),
            "Up" => self.handle_release(2, 0x20),
            "Down" => self.handle_release(3, 0x20),
            _ => (),
        }
    }

    fn handle_press(&mut self, bit_position: u8, group_bit: u8) {
        if self.is_group_selected(group_bit) {
            let was_pressed = self.joy & (1 << bit_position) == 0;
            if !was_pressed {
                self.joy &= !(1 << bit_position);
                if let Some(ref cpu) = self.cpu {
                    cpu.borrow_mut().request_interrupt(0b00010000);
                }
            }
        }
    }

    fn handle_release(&mut self, bit_position: u8, group_bit: u8) {
        if self.is_group_selected(group_bit) {
            self.joy |= 1 << bit_position;
        }
    }

    fn is_group_selected(&self, group_bit: u8) -> bool {
        self.joy & group_bit == 0
    }
}
