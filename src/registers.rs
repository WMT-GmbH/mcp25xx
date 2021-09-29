use modular_bitfield::prelude::*;

pub trait Register {
    const ADDRESS: u8;
}

pub trait Modify {}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RXB0CTRL {
    #[skip(setters)]
    pub filhit0: bool,
    #[skip(setters)]
    pub bukt1: bool,
    pub bukt: bool,
    #[skip(setters)]
    pub rxrtr: bool,
    #[skip]
    __: B1,
    pub rxm0: bool,
    pub rxm1: bool,
    #[skip]
    __: B1,
}

impl Register for RXB0CTRL {
    const ADDRESS: u8 = 0x60;
}

impl Modify for RXB0CTRL {}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RXB1CTRL {
    #[skip(setters)]
    pub filhit0: bool,
    #[skip(setters)]
    pub filhit1: bool,
    #[skip(setters)]
    pub filhit2: bool,
    #[skip(setters)]
    pub rxrtr: bool,
    #[skip]
    __: B1,
    pub rxm0: bool,
    pub rxm1: bool,
    #[skip]
    __: B1,
}
impl Register for RXB1CTRL {
    const ADDRESS: u8 = 0x70;
}
impl Modify for RXB1CTRL {}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct BFPCTRL {
    pub b0bfm: bool,
    pub b1bfm: bool,
    pub b0bfe: bool,
    pub b1bfe: bool,
    pub b0bfs: bool,
    pub b1bfs: bool,
    #[skip]
    __: B2,
}

impl Register for BFPCTRL {
    const ADDRESS: u8 = 0x0C;
}
impl Modify for BFPCTRL {}

#[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub struct CANCTRL {
    pub clkpre: CLKPRE,
    pub clken: bool,
    #[skip]
    __: B1,
    pub abat: bool,
    pub reqop: REQOP,
}

#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub struct CANCTRL {
    pub clkpre: CLKPRE,
    pub clken: bool,
    pub osm: bool,
    pub abat: bool,
    pub reqop: REQOP,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 3]
pub enum REQOP {
    NormalOperation = 0b000,
    Sleep = 0b001,
    Loopback = 0b010,
    ListenOnly = 0b011,
    Configuration = 0b100,
    Invalid1 = 0b101,
    Invalid2 = 0b110,
    Invalid3 = 0b111,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum CLKPRE {
    SystemClockDiv1 = 0b000,
    SystemClockDiv2 = 0b001,
    SystemClockDiv4 = 0b010,
    SystemClockDiv8 = 0b011,
}

impl Register for CANCTRL {
    const ADDRESS: u8 = 0x0F;
}
impl Modify for CANCTRL {}

#[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
impl Default for CANCTRL {
    fn default() -> Self {
        0b1110_0111.into()
    }
}

#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
impl Default for CANCTRL {
    fn default() -> Self {
        0b1000_0111.into()
    }
}

#[bitfield]
#[derive(Copy, Clone, Debug, Default)]
pub struct CNF {
    pub cnf3: CNF3,
    pub cnf2: CNF2,
    pub cnf1: CNF1,
}

impl Register for CNF {
    const ADDRESS: u8 = CNF3::ADDRESS;
}

#[bitfield]
#[derive(BitfieldSpecifier, Copy, Clone, Debug, Default)]
pub struct CNF1 {
    pub brp: B6,
    pub sjw: B2,
}

impl Register for CNF1 {
    const ADDRESS: u8 = 0x2A;
}
impl Modify for CNF1 {}

#[bitfield]
#[derive(BitfieldSpecifier, Copy, Clone, Debug, Default)]
pub struct CNF2 {
    pub prseg: B3,
    pub phseg1: B3,
    pub sam: bool,
    pub btlmode: bool,
}

impl Register for CNF2 {
    const ADDRESS: u8 = 0x29;
}
impl Modify for CNF2 {}

#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[bitfield]
#[derive(BitfieldSpecifier, Copy, Clone, Debug, Default)]
pub struct CNF3 {
    pub phseg2: B3,
    #[skip]
    __: B3,
    pub wakfil: bool,
    sof: bool,
}

#[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
#[bitfield]
#[derive(BitfieldSpecifier, Copy, Clone, Debug, Default)]
pub struct CNF3 {
    pub phseg2: B3,
    #[skip]
    __: B3,
    pub wakfil: bool,
    #[skip]
    __: B1,
}

impl Register for CNF3 {
    const ADDRESS: u8 = 0x28;
}
impl Modify for CNF3 {}
