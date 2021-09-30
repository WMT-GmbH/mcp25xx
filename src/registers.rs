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
/// Can Control Register
pub struct CANCTRL {
    /// CLKOUT Pin Prescaler
    pub clkpre: CLKPRE,
    ///  CLKOUT Pin Enable
    pub clken: bool,
    #[skip]
    __: B1,
    /// Abort All Pending Transmissions
    pub abat: bool,
    /// Request Operation Mode
    pub reqop: REQOP,
}

#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
/// Can Control Register
pub struct CANCTRL {
    /// CLKOUT Pin Prescaler
    pub clkpre: CLKPRE,
    ///  CLKOUT Pin Enable
    pub clken: bool,
    /// One-Shot Mode
    pub osm: bool,
    /// Abort All Pending Transmissions
    pub abat: bool,
    /// Request Operation Mode
    pub reqop: REQOP,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 3]
/// Request Operation mode
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
/// CLKOUT Pin Prescaler
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
/// Configuration Register
pub struct CNF {
    /// Configuration 3 Register
    pub cnf3: CNF3,
    /// Configuration 2 Register
    pub cnf2: CNF2,
    /// Configuration 1 Register
    pub cnf1: CNF1,
}

impl Register for CNF {
    const ADDRESS: u8 = CNF3::ADDRESS;
}

#[bitfield]
#[repr(u8)]
#[derive(BitfieldSpecifier, Copy, Clone, Debug, Default)]
/// Configuration 1 Register
pub struct CNF1 {
    /// Baud Rate Prescaler
    pub brp: B6,
    /// Synchronization Jump Width Length
    pub sjw: B2,
}

impl Register for CNF1 {
    const ADDRESS: u8 = 0x2A;
}
impl Modify for CNF1 {}

#[bitfield]
#[repr(u8)]
#[derive(BitfieldSpecifier, Copy, Clone, Debug, Default)]
/// Configuration 2 Register
pub struct CNF2 {
    /// Propagation Segment Length
    pub prseg: B3,
    /// PS1 Length
    pub phseg1: B3,
    /// Sample Point Configuration
    pub sam: bool,
    /// PS2 Bit Time Length
    pub btlmode: bool,
}

impl Register for CNF2 {
    const ADDRESS: u8 = 0x29;
}
impl Modify for CNF2 {}

#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[bitfield]
#[repr(u8)]
#[derive(BitfieldSpecifier, Copy, Clone, Debug, Default)]
/// Configuration 3 Register
pub struct CNF3 {
    /// PS2 Length
    pub phseg2: B3,
    #[skip]
    __: B3,
    /// Wake-up Filter
    pub wakfil: bool,
    /// Start-of-Frame Signal
    pub sof: bool,
}

#[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
#[bitfield]
#[repr(u8)]
#[derive(BitfieldSpecifier, Copy, Clone, Debug, Default)]
/// Configuration 3 Register
pub struct CNF3 {
    /// PS2 Length
    pub phseg2: B3,
    #[skip]
    __: B3,
    /// Wake-up Filter
    pub wakfil: bool,
    #[skip]
    __: B1,
}

impl Register for CNF3 {
    const ADDRESS: u8 = 0x28;
}
impl Modify for CNF3 {}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
/// Data Length Code Register
pub struct DLC {
    /// Data Length Code
    pub dlc: B4,
    #[skip]
    __: B2,
    /// Remote Transmission Request
    pub rtr: bool,
    #[skip]
    __: B1,
}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
/// Transmit Buffer N Control Register
pub struct TXB0CTRL {
    /// Transmit Buffer Priority
    pub txp: B2,
    #[skip]
    __: B1,
    /// Message Transmit Request
    pub txreq: bool,
    /// Transmission Error Detected
    pub txerr: bool,
    /// : Message Lost Arbitration
    pub mloa: bool,
    /// Message Aborted Flag
    pub abtf: bool,
    #[skip]
    __: B1,
}

impl Register for TXB0CTRL {
    const ADDRESS: u8 = 0x30;
}

impl Modify for TXB0CTRL {}
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
/// Transmit Buffer N Control Register
pub struct TXB1CTRL {
    /// Transmit Buffer Priority
    pub txp: B2,
    #[skip]
    __: B1,
    /// Message Transmit Request
    pub txreq: bool,
    /// Transmission Error Detected
    pub txerr: bool,
    /// : Message Lost Arbitration
    pub mloa: bool,
    /// Message Aborted Flag
    pub abtf: bool,
    #[skip]
    __: B1,
}

impl Register for TXB1CTRL {
    const ADDRESS: u8 = 0x40;
}

impl Modify for TXB1CTRL {}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
/// Transmit Buffer N Control Register
pub struct TXB2CTRL {
    /// Transmit Buffer Priority
    pub txp: B2,
    #[skip]
    __: B1,
    /// Message Transmit Request
    pub txreq: bool,
    /// Transmission Error Detected
    pub txerr: bool,
    /// : Message Lost Arbitration
    pub mloa: bool,
    /// Message Aborted Flag
    pub abtf: bool,
    #[skip]
    __: B1,
}

impl Register for TXB2CTRL {
    const ADDRESS: u8 = 0x50;
}

impl Modify for TXB2CTRL {}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
/// Read Status Response Bitfield
pub struct ReadStatusResponse {
    pub tx2if: bool,
    pub txreq2: bool,
    pub tx1if: bool,
    pub txreq1: bool,
    pub tx0if: bool,
    pub txreq0: bool,
    pub rx1if: bool,
    pub rx0if: bool,
}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CANINTF {
    pub rx0if: bool,
    pub rx1if: bool,
    pub tx0if: bool,
    pub tx1if: bool,
    pub tx2if: bool,
    pub errif: bool,
    pub wakif: bool,
    pub merrf: bool,
}

impl Register for CANINTF {
    const ADDRESS: u8 = 0x2C;
}

impl Modify for CANINTF {}
