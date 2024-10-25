pub struct BOOTROM {
    pub boot_enabled: bool,
    boot_room: [u8; 256],
}

impl BOOTROM {
    pub fn new() -> Self {
        let mut bootrom_contents: [u8; 256] = [0x00; 256];

        bootrom_contents[0x00] = 0x31;
        bootrom_contents[0x01] = 0xFE;
        bootrom_contents[0x02] = 0xFF;
        bootrom_contents[0x03] = 0xC3;
        bootrom_contents[0x04] = 0xFC;
        bootrom_contents[0x05] = 0x00;
        bootrom_contents[0xFC] = 0x3E;
        bootrom_contents[0xFD] = 0x01;
        bootrom_contents[0xFE] = 0xE0;
        bootrom_contents[0xFF] = 0x50;

        BOOTROM {
            boot_enabled: true,
            boot_room: bootrom_contents,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.boot_room[address as usize]
    }

    pub fn disable(&mut self) {
        self.boot_enabled = false;
    }
}
