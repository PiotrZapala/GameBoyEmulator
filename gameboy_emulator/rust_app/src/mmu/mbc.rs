use crate::cartridge::CARTRIDGE;

pub trait MBC {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}

pub fn create_mbc(cartridge: CARTRIDGE) -> Box<dyn MBC> {
    match cartridge.cartridge_type {
        0x00 => Box::new(NMBC::new(cartridge.rom, None, false)),                              // ROM
        0x08 => Box::new(NMBC::new(cartridge.rom, Some(cartridge.ram_size as usize), false)), // ROM + RAM
        0x09 => Box::new(NMBC::new(cartridge.rom, Some(cartridge.ram_size as usize), true)),  // ROM + RAM + BATTERY
        0x01 => Box::new(MBC1::new(cartridge.rom, None, false)),                              // MBC1
        0x02 => Box::new(MBC1::new(cartridge.rom, Some(cartridge.ram_size as usize), false)), // MBC1 + RAM
        0x03 => Box::new(MBC1::new(cartridge.rom, Some(cartridge.ram_size as usize), true)),  // MBC1 + RAM + BATTERY
        _ => panic!("Unsupported MBC type"),
    }
}

pub struct NMBC {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    ram_battery: bool,
}

impl NMBC {
    pub fn new(rom: Vec<u8>, ram_size: Option<usize>, included_ram_battery: bool) -> Self {
        let ram = ram_size.map(|size| vec![0; size]);
        NMBC {
            rom,
            ram,
            ram_battery: included_ram_battery,
        }
    }
}

impl MBC for NMBC {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0xA000..=0xBFFF => {
                if let Some(ref ram) = self.ram {
                    ram[(address - 0xA000) as usize]
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if let Some(ref mut ram) = self.ram {
            ram[(address - 0xA000) as usize] = value;
        }
    }
}

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    ram_battery: bool,
    rom_bank: u8,
    ram_bank: u8,
    mode: bool,
    ram_enabled: bool,
}

impl MBC1 {
    pub fn new(rom: Vec<u8>, ram_size: Option<usize>, included_ram_battery: bool) -> Self {
        let ram = ram_size.map(|size| vec![0; size]);
        MBC1 {
            rom,
            ram,
            ram_battery: included_ram_battery,
            rom_bank: 1,
            ram_bank: 0,
            mode: false,
            ram_enabled: false,
        }
    }
}

impl MBC for MBC1 {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                self.rom[address as usize]
            }
            0x4000..=0x7FFF => {
                let bank_address = address as usize - 0x4000;
                let bank_offset = self.rom_bank as usize * 0x4000;
                self.rom[bank_address + bank_offset]
            }
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if let Some(ref ram) = self.ram {
                        let bank_address = address as usize - 0xA000;
                        let bank_offset = self.ram_bank as usize * 0x2000;
                        ram[bank_address + bank_offset]
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => panic!("Attempted to read from an invalid memory address: {:04X}", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {

            }
            0x2000..=0x3FFF => {

            }
            0x4000..=0x5FFF => {

            }
            0x6000..=0x7FFF => {

            }
            0xA000..=0xBFFF => {

            }
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }
}