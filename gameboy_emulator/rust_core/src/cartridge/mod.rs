pub struct CARTRIDGE {
    pub rom: Vec<u8>,                // Cała zawartość ROM
    pub entry_point: [u8; 4],        // 0x0100-0x0103
    pub nintendo_logo: [u8; 48],     // 0x0104-0x0133
    pub title: [u8; 15],             // 0x0134-0x0143
    pub manufacturer_code: [u8; 4],  // 0x013F-0x0142
    pub cgb_flag: u8,                // 0x0143
    pub new_licensee_code: [u8; 2],  // 0x0144-0x0145
    pub sgb_flag: u8,                // 0x0146
    pub cartridge_type: u8,          // 0x0147
    pub rom_size: u8,                // 0x0148
    pub ram_size: Option<u32>,       // 0x0149
    pub destination_code: u8,        // 0x014A
    pub old_licensee_code: u8,       // 0x014B
    pub mask_rom_version_number: u8, // 0x014C
    pub header_checksum: u8,         // 0x014D
    pub global_checksum: [u8; 2],    // 0x014E-0x014F
    pub saved_ram: Option<Vec<u8>>,
}

impl CARTRIDGE {
    pub fn new(data: Vec<u8>, saved_ram: Option<Vec<u8>>) -> Self {
        let ram_size = match data.get(0x0149) {
            Some(0x00) => None,             
            Some(0x01) => Some(2 * 1024),   
            Some(0x02) => Some(8 * 1024),   
            Some(0x03) => Some(32 * 1024),  
            Some(0x04) => Some(128 * 1024), 
            Some(0x05) => Some(64 * 1024),  
            _ => None,                      
        };

        Self {
            entry_point: [data[0x0100], data[0x0101], data[0x0102], data[0x0103]],
            nintendo_logo: data[0x0104..0x0134].try_into().unwrap(),
            title: data[0x0134..0x0143].try_into().unwrap(),
            manufacturer_code: data[0x013F..0x0143].try_into().unwrap(),
            cgb_flag: data[0x0143],
            new_licensee_code: [data[0x0144], data[0x0145]],
            sgb_flag: data[0x0146],
            cartridge_type: data[0x0147],
            rom_size: data[0x0148],
            ram_size,
            destination_code: data[0x014A],
            old_licensee_code: data[0x014B],
            mask_rom_version_number: data[0x014C],
            header_checksum: data[0x014D],
            global_checksum: [data[0x014E], data[0x014F]],
            rom: data,
            saved_ram,
        }
    }

    pub fn decode_title(&self) -> String {
        let decoded_title: String = self.title
            .iter()
            .take_while(|&&c| c != 0) 
            .map(|&c| c as char)       
            .collect();               

        decoded_title
    }
}
