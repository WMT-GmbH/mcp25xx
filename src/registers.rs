#![allow(unused_parens)] // FIXME: remove after https://github.com/modular-bitfield/modular-bitfield/pull/125

use modular_bitfield::prelude::*;

/// 8 bit Register
pub trait Register: From<u8> + Into<u8> {
    /// Address of the register
    const ADDRESS: u8;
}

/// Marker trait for Registers that support the `Modify` instruction.
pub trait Modify {}

/// Receive Buffer 0 Control Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RXB0CTRL {
    #[skip(setters)]
    /// Filter Hit bit (indicates which acceptance filter enabled reception of message)
    /// Note: If a rollover from RXB0 to RXB1 occurs, the `filhit0` bit will reflect the filter that accepted the message that rolled over
    pub filhit: B1,
    #[skip(setters)]
    /// Read-Only Copy of BUKT bit (used internally)
    pub bukt1: bool,
    /// Rollover Enable
    pub bukt: bool,
    #[skip(setters)]
    /// Received Remote Transfer Request
    pub rxrtr: bool,
    #[skip]
    __: B1,
    /// Receive Buffer Operating Mode
    pub rxm: RXM,
    #[skip]
    __: B1,
}

/// Receive Buffer 1 Control Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RXB1CTRL {
    #[skip(setters)]
    /// Filter Hit bits (indicates which acceptance filter enabled reception of message)
    pub filhit: B3,
    #[skip(setters)]
    /// Received Remote Transfer Request bit
    pub rxrtr: bool,
    #[skip]
    __: B1,
    /// Receive Buffer Operating Mode
    pub rxm: RXM,
    #[skip]
    __: B1,
}

/// Receive Buffer Operating Mode
#[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
#[derive(Specifier, Copy, Clone, Debug)]
#[bits = 2]
pub enum RXM {
    /// Receive all valid messages using either standard or extended identifiers that meet filter criteria
    Filter = 0b00,
    /// Receive only valid messages with standard identifiers that meet filter criteria
    FilterStandard = 0b01,
    /// Receive only valid messages with extended identifiers that meet filter criteria
    FilterExtended = 0b10,
    /// Turn mask/filters off; receive any message
    ReceiveAny = 0b11,
}

/// Receive Buffer Operating Mode
#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[derive(BitfieldSpecifier, Copy, Clone, Debug)]
#[bits = 2]
pub enum RXM {
    /// Receive all valid messages using either standard or extended identifiers that meet filter criteria
    Filter = 0b00,
    Reserved1 = 0b01,
    Reserved2 = 0b10,
    /// Turn mask/filters off; receive any message
    ReceiveAny = 0b11,
}

/// Can Control Register
#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub struct CANCTRL {
    /// CLKOUT Pin Prescaler
    pub clkpre: CLKPRE,
    ///  CLKOUT Pin Enable
    pub clken: bool,
    #[cfg_attr(doc, doc(cfg(any(feature = "mcp2515", feature = "mcp25625"))))]
    /// One-Shot Mode
    pub osm: bool,
    /// Abort All Pending Transmissions
    pub abat: bool,
    /// Request Operation Mode
    pub reqop: OperationMode,
}

/// Can Control Register
#[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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
    pub reqop: OperationMode,
}

/// Request Operation mode
#[derive(Specifier, Copy, Clone, Debug)]
#[bits = 3]
pub enum OperationMode {
    NormalOperation = 0b000,
    Sleep = 0b001,
    Loopback = 0b010,
    ListenOnly = 0b011,
    Configuration = 0b100,
    Invalid1 = 0b101,
    Invalid2 = 0b110,
    Invalid3 = 0b111,
}

/// CLKOUT Pin Prescaler
#[derive(Specifier, Copy, Clone, Debug)]
#[bits = 2]
pub enum CLKPRE {
    SystemClockDiv1 = 0b000,
    SystemClockDiv2 = 0b001,
    SystemClockDiv4 = 0b010,
    SystemClockDiv8 = 0b011,
}

/// ```ignore
/// CANCTRL {
///     clkpre: CLKPRE::SystemClockDiv8,
///     clken: true,
///     osm: false, // field not present on MCP2510
///     abat: false,
///     reqop: OperationMode::Configuration,
/// }
/// ```
impl Default for CANCTRL {
    fn default() -> Self {
        0b1000_0111.into()
    }
}

/// CAN Status Register
///
/// Note: Read only
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub struct CANSTAT {
    #[skip]
    __: B1,
    /// Interrupt Flag Code
    pub icod: InterruptFlagCode,
    #[skip]
    __: B1,
    /// Operation Mode
    pub opmod: OperationMode,
}

/// Interrupt Flag Code
#[derive(Specifier, Copy, Clone, Debug)]
#[bits = 3]
pub enum InterruptFlagCode {
    NoInterrupt = 0b000,
    ErrorInterrupt = 0b001,
    WakeUpInterrupt = 0b010,
    TXB0Interrupt = 0b011,
    TXB1Interrupt = 0b100,
    TXB2Interrupt = 0b101,
    RXB0Interrupt = 0b110,
    RXB1Interrupt = 0b111,
}

impl Default for CANSTAT {
    fn default() -> Self {
        0b1000_0000.into()
    }
}

/// Configuration Registers
///
/// Note: Write operations require Configuration mode
#[derive(Copy, Clone, Debug, Default)]
pub struct CNF {
    /// Configuration 3 Register
    pub cnf3: CNF3,
    /// Configuration 2 Register
    pub cnf2: CNF2,
    /// Configuration 1 Register
    pub cnf1: CNF1,
}

impl CNF {
    pub const fn from_bytes(bytes: [u8; 3]) -> Self {
        CNF {
            cnf3: CNF3::from_bytes([bytes[0]]),
            cnf2: CNF2::from_bytes([bytes[1]]),
            cnf1: CNF1::from_bytes([bytes[2]]),
        }
    }
    pub const fn into_bytes(self) -> [u8; 3] {
        [
            self.cnf3.into_bytes()[0],
            self.cnf2.into_bytes()[0],
            self.cnf1.into_bytes()[0],
        ]
    }
}

/// Configuration 1 Register
///
/// Note: Write operations require Configuration mode
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CNF1 {
    /// Baud Rate Prescaler
    pub brp: B6,
    /// Synchronization Jump Width Length
    pub sjw: B2,
}

/// Configuration 2 Register
///
/// Note: Write operations require Configuration mode
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
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

/// Configuration 3 Register
///
/// Note: Write operations require Configuration mode
#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CNF3 {
    /// PS2 Length
    /// Note: Minimum valid setting is 1
    pub phseg2: B3,
    #[skip]
    __: B3,
    /// Wake-up Filter
    pub wakfil: bool,
    #[cfg_attr(doc, doc(cfg(any(feature = "mcp2515", feature = "mcp25625"))))]
    /// Start-of-Frame Signal
    pub sof: bool,
}

/// Configuration 3 Register
///
/// Note: Write operations require Configuration mode
#[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
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

/// Data Length Code Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
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

/// Transmit Buffer 0 Control Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
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

/// Transmit Buffer 1 Control Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
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

/// Transmit Buffer 2 Control Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
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

/// Interrupt Enable Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CANINTE {
    /// Receive Buffer 0 Full Interrupt Enable
    pub rx0ie: bool,
    /// Receive Buffer 1 Full Interrupt Enable
    pub rx1ie: bool,
    /// Transmit Buffer 0 Empty Interrupt Enable
    pub tx0ie: bool,
    /// Transmit Buffer 1 Empty Interrupt Enable
    pub tx1ie: bool,
    /// Transmit Buffer 2 Empty Interrupt Enable
    pub tx2ie: bool,
    /// Error Interrupt Enable (multiple sources in the [`EFLG`] register)
    pub errie: bool,
    /// Wake-up Interrupt Enable
    pub wakie: bool,
    /// Message Error Interrupt Enable
    pub merre: bool,
}

/// Interrupt Flag Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CANINTF {
    /// Receive Buffer 0 Full Interrupt Flag
    pub rx0if: bool,
    /// Receive Buffer 1 Full Interrupt Flag
    pub rx1if: bool,
    /// Transmit Buffer 0 Empty Interrupt Flag
    pub tx0if: bool,
    /// Transmit Buffer 1 Empty Interrupt Flag
    pub tx1if: bool,
    /// Transmit Buffer 2 Empty Interrupt Flag
    pub tx2if: bool,
    /// Error Interrupt Flag (multiple sources in the [`EFLG`] register)
    pub errif: bool,
    /// Wake-up Interrupt Flag
    pub wakif: bool,
    /// Message Error Interrupt Flag
    pub merrf: bool,
}

/// Error Flag Register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct EFLG {
    /// Error Warning Flag bit
    pub ewarn: bool,
    /// Receive Error Warning Flag bit
    pub rxwar: bool,
    /// Transmit Error Warning Flag bit
    pub txwar: bool,
    /// Receive Error-Passive Flag bit
    pub rxep: bool,
    /// Transmit Error-Passive Flag bit
    pub txep: bool,
    /// Bus-Off Error Flag bit
    pub txbo: bool,
    /// Receive Buffer 0 Overflow Flag bit
    pub rx0ovr: bool,
    /// Receive Buffer 1 Overflow Flag bit
    pub rx1ovr: bool,
}

/// RXnBF Pin Control and Status Register
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

/// TXnRTS Pin Control and Status Register
///
/// Note: Write operations require Configuration mode
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct TXRTSCTRL {
    pub b0rtsm: bool,
    pub b1rtsm: bool,
    pub b2rtsm: bool,
    pub b0rts: bool,
    pub b1rts: bool,
    pub b2rts: bool,
    #[skip]
    __: B2,
}

/// Transmit Error Counter Register
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct TEC(pub u8);

impl From<u8> for TEC {
    fn from(val: u8) -> Self {
        TEC(val)
    }
}
impl From<TEC> for u8 {
    fn from(val: TEC) -> Self {
        val.0
    }
}

/// Receive Error Counter Register
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct REC(pub u8);

impl From<u8> for REC {
    fn from(val: u8) -> Self {
        REC(val)
    }
}
impl From<REC> for u8 {
    fn from(val: REC) -> Self {
        val.0
    }
}

/// Read Status Response Bitfield
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ReadStatusResponse {
    /// Receive Buffer 0 Full Interrupt Flag
    pub rx0if: bool,
    /// Receive Buffer 1 Full Interrupt Flag
    pub rx1if: bool,
    /// Message Transmit Request bit
    pub txreq0: bool,
    /// Transmit Buffer 0 Empty Interrupt Flag
    pub tx0if: bool,
    /// Message Transmit Request bit
    pub txreq1: bool,
    /// Transmit Buffer 1 Empty Interrupt Flag
    pub tx1if: bool,
    /// Message Transmit Request bit
    pub txreq2: bool,
    /// Transmit Buffer 2 Empty Interrupt Flag
    pub tx2if: bool,
}

/// Read Status Response Bitfield
#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[cfg_attr(doc, doc(cfg(any(feature = "mcp2515", feature = "mcp25625"))))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RxStatusResponse {
    pub filter_match: FilterMatch,
    pub is_remote: bool,
    pub is_extended: bool,
    #[skip]
    __: B1,
    /// Receive Buffer 0 Full Interrupt Flag
    pub rx0if: bool,
    /// Receive Buffer 1 Full Interrupt Flag
    pub rx1if: bool,
}

/// The filter that matched the received message
#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
#[cfg_attr(doc, doc(cfg(any(feature = "mcp2515", feature = "mcp25625"))))]
#[derive(BitfieldSpecifier, Copy, Clone, Debug)]
#[bits = 3]
pub enum FilterMatch {
    RXF0,
    RXF1,
    RXF2,
    RXF3,
    RXF4,
    RXF5,
    RXF0Rollover,
    RXF1Rollover,
}

impl Register for RXB0CTRL {
    const ADDRESS: u8 = 0x60;
}
impl Register for RXB1CTRL {
    const ADDRESS: u8 = 0x70;
}
impl Register for CANCTRL {
    const ADDRESS: u8 = 0x0F;
}
impl Register for CANSTAT {
    const ADDRESS: u8 = 0x0E;
}
impl Register for CNF1 {
    const ADDRESS: u8 = 0x2A;
}
impl Register for CNF2 {
    const ADDRESS: u8 = 0x29;
}
impl Register for CNF3 {
    const ADDRESS: u8 = 0x28;
}
impl Register for TXB0CTRL {
    const ADDRESS: u8 = 0x30;
}
impl Register for TXB1CTRL {
    const ADDRESS: u8 = 0x40;
}
impl Register for TXB2CTRL {
    const ADDRESS: u8 = 0x50;
}
impl Register for CANINTE {
    const ADDRESS: u8 = 0x2B;
}
impl Register for CANINTF {
    const ADDRESS: u8 = 0x2C;
}
impl Register for EFLG {
    const ADDRESS: u8 = 0x2D;
}
impl Register for BFPCTRL {
    const ADDRESS: u8 = 0x0C;
}
impl Register for TXRTSCTRL {
    const ADDRESS: u8 = 0x0D;
}
impl Register for TEC {
    const ADDRESS: u8 = 0x1C;
}
impl Register for REC {
    const ADDRESS: u8 = 0x1D;
}

impl Modify for CANCTRL {}
impl Modify for CNF1 {}
impl Modify for CNF2 {}
impl Modify for CNF3 {}
impl Modify for TXB0CTRL {}
impl Modify for TXB1CTRL {}
impl Modify for TXB2CTRL {}
impl Modify for RXB0CTRL {}
impl Modify for RXB1CTRL {}
impl Modify for CANINTE {}
impl Modify for CANINTF {}
impl Modify for EFLG {}
impl Modify for BFPCTRL {}
impl Modify for TXRTSCTRL {}
