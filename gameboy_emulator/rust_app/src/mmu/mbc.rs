use crate::cartridge::CARTRIDGE;

pub trait MBC: Send {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}

pub fn create_mbc(cartridge: CARTRIDGE) -> Box<dyn MBC> {
    let ram_size = cartridge.ram_size.map(|size| size as usize);
    match cartridge.cartridge_type {
        0x00 => Box::new(NMBC::new(cartridge.rom, None, false)),      // ROM
        0x08 => Box::new(NMBC::new(cartridge.rom, ram_size, false)),           // ROM + RAM
        0x09 => Box::new(NMBC::new(cartridge.rom, ram_size, true)),            // ROM + RAM + BATTERY
        0x01 => Box::new(MBC1::new(cartridge.rom, None, false)),      // MBC1
        0x02 => Box::new(MBC1::new(cartridge.rom, ram_size, false)),           // MBC1 + RAM
        0x03 => Box::new(MBC1::new(cartridge.rom, ram_size, true)),            // MBC1 + RAM + BATTERY
        0x11 => Box::new(MBC3::new(cartridge.rom, None, false)),     // MBC3
        0x12 => Box::new(MBC3::new(cartridge.rom, ram_size, false)),           // MBC3 + RAM
        0x13 => Box::new(MBC3::new(cartridge.rom, ram_size, true)),            // MBC3 + RAM + BATTERY
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
            0x0000..=0x7FFF => {
                self.rom[address as usize]},
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
                let bank_offset = 0x4000 * self.rom_bank as usize;
                let address_in_bank = address as usize - 0x4000;
                self.rom[bank_offset + address_in_bank]
            }
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if let Some(ref ram) = self.ram {
                        let bank_offset = (self.ram_bank as usize) * 0x2000;
                        let bank_address = address as usize - 0xA000;
                        ram[bank_offset + bank_address]
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x2000..=0x3FFF => {
                let bank = value & 0x1F;
                self.rom_bank = if bank == 0 { 1 } else { bank };
            }
            0x4000..=0x5FFF => {
                self.ram_bank = value & 0x03;
            }
            0x6000..=0x7FFF => {
                self.mode = (value & 0x01) != 0;
            }
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if let Some(ref mut ram) = self.ram {
                        let bank_offset = (self.ram_bank as usize) * 0x2000;
                        let bank_address = address as usize - 0xA000;
                        ram[bank_offset + bank_address] = value;
                    }
                }
            }
            _ => (),
        }
    }
}

pub struct MBC3 {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    ram_battery: bool,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
}

impl MBC3 {
    pub fn new(rom: Vec<u8>, ram_size: Option<usize>, included_ram_battery: bool) -> Self {
        let ram = ram_size.map(|size| vec![0; size]);
        MBC3 {
            rom,
            ram,
            ram_battery: included_ram_battery,
            rom_bank: 1,      
            ram_bank: 0,      
            ram_enabled: false,
        }
    }
}

impl MBC for MBC3 {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                self.rom[address as usize]},
            
            0x4000..=0x7FFF => {
                let bank_offset = (self.rom_bank as usize) * 0x4000;
                let bank_address = address as usize - 0x4000;
                self.rom[bank_offset + bank_address]
            },
            
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if let Some(ref ram) = self.ram {
                        let bank_offset = (self.ram_bank as usize) * 0x2000;
                        let bank_address = address as usize - 0xA000;
                        ram[bank_offset + bank_address]
                    } else {
                        0
                    }
                } else {
                    0
                }
            },
            _ => 0,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            },
            
            0x2000..=0x3FFF => {
                let bank = value & 0x7F;
                self.rom_bank = if bank == 0 { 1 } else { bank };
            },
            
            0x4000..=0x5FFF => {
                self.ram_bank = value & 0x03;
            },
            
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if let Some(ref mut ram) = self.ram {
                        let bank_offset = (self.ram_bank as usize) * 0x2000;
                        let bank_address = address as usize - 0xA000;
                        ram[bank_offset + bank_address] = value;
                    }
                }
            },
            _ => (),
        }
    }
}
