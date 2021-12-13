// values taken from https://github.com/autowp/arduino-mcp2515/blob/master/mcp2515.h
// cnf2.btlmode = true
// cnf3.wakfil = false
// cnf2.sam = ?

/// Preconfigured CNF registers for 8 Mhz oscillators
pub mod clock_8mhz {
    use crate::registers::CNF;

    // pub const CNF_1000K_BPS: CNF = CNF::from_bytes([0x80, 0x80, 0x00]); invalid see https://github.com/autowp/arduino-mcp2515/issues/44
    pub const CNF_500K_BPS: CNF = CNF::from_bytes([0x82, 0x90, 0x00]);
    pub const CNF_250K_BPS: CNF = CNF::from_bytes([0x85, 0xB1, 0x00]);
    pub const CNF_200K_BPS: CNF = CNF::from_bytes([0x86, 0xB4, 0x00]);
    pub const CNF_125K_BPS: CNF = CNF::from_bytes([0x85, 0xB1, 0x01]);
    pub const CNF_100K_BPS: CNF = CNF::from_bytes([0x86, 0xB4, 0x01]);
    pub const CNF_80K_BPS: CNF = CNF::from_bytes([0x87, 0xBF, 0x01]);
    pub const CNF_50K_BPS: CNF = CNF::from_bytes([0x86, 0xB4, 0x03]);
    pub const CNF_40K_BPS: CNF = CNF::from_bytes([0x87, 0xBF, 0x03]);
    pub const CNF_33K3_BPS: CNF = CNF::from_bytes([0x85, 0xE2, 0x47]);
    pub const CNF_31K25_BPS: CNF = CNF::from_bytes([0x84, 0xA4, 0x07]);
    pub const CNF_20K_BPS: CNF = CNF::from_bytes([0x87, 0xBF, 0x07]);
    pub const CNF_10K_BPS: CNF = CNF::from_bytes([0x87, 0xBF, 0x0F]);
    pub const CNF_5K_BPS: CNF = CNF::from_bytes([0x87, 0xBF, 0x1F]);
}

/// Preconfigured CNF registers for 16 Mhz oscillators
pub mod clock_16mhz {
    use crate::registers::CNF;

    pub const CNF_1000K_BPS: CNF = CNF::from_bytes([0x82, 0xD0, 0x00]);
    pub const CNF_500K_BPS: CNF = CNF::from_bytes([0x86, 0xF0, 0x00]);
    pub const CNF_250K_BPS: CNF = CNF::from_bytes([0x85, 0xF1, 0x41]);
    pub const CNF_200K_BPS: CNF = CNF::from_bytes([0x87, 0xFA, 0x01]);
    pub const CNF_125K_BPS: CNF = CNF::from_bytes([0x86, 0xF0, 0x03]);
    pub const CNF_100K_BPS: CNF = CNF::from_bytes([0x87, 0xFA, 0x03]);
    pub const CNF_80K_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x03]);
    pub const CNF_83K3_BPS: CNF = CNF::from_bytes([0x07, 0xBE, 0x03]);
    pub const CNF_50K_BPS: CNF = CNF::from_bytes([0x87, 0xFA, 0x07]);
    pub const CNF_40K_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x07]);
    pub const CNF_33K3_BPS: CNF = CNF::from_bytes([0x85, 0xF1, 0x4E]);
    pub const CNF_20K_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x0F]);
    pub const CNF_10K_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x1F]);
    pub const CNF_5K_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x3F]);
}

/// Preconfigured CNF registers for 20 Mhz oscillators
pub mod clock_20mhz {
    use crate::registers::CNF;

    pub const CNF_1000K_BPS: CNF = CNF::from_bytes([0x82, 0xD9, 0x00]);
    pub const CNF_500K_BPS: CNF = CNF::from_bytes([0x87, 0xFA, 0x00]);
    pub const CNF_250K_BPS: CNF = CNF::from_bytes([0x86, 0xFB, 0x41]);
    pub const CNF_200K_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x01]);
    pub const CNF_125K_BPS: CNF = CNF::from_bytes([0x87, 0xFA, 0x03]);
    pub const CNF_100K_BPS: CNF = CNF::from_bytes([0x87, 0xFA, 0x04]);
    pub const CNF_83K3_BPS: CNF = CNF::from_bytes([0x87, 0xFE, 0x04]);
    pub const CNF_80K_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x04]);
    pub const CNF_50K_BPS: CNF = CNF::from_bytes([0x87, 0xFA, 0x09]);
    pub const CNF_40K_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x09]);
    pub const CNF_33K3_BPS: CNF = CNF::from_bytes([0x87, 0xFF, 0x0B]);
}
