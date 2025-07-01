//! `no_std` library for the MCP2510, MCP2515 and MCP25625 CAN controller chips.
//!
//! API is implemented in terms of of the [`embedded_hal`] and [`embedded_can`] traits.
//!
//! Activating the `mcp2515` or `mcp25625` feature will enable
//! additional registers and instructions the MCP2510 does not support.
//!
//! # Example
//!
//! ```
//! use embedded_can::nb::Can;
//! use embedded_can::{Frame, StandardId};
//! use mcp25xx::bitrates::clock_16mhz::CNF_500K_BPS;
//! # use mcp25xx::doctesthelper::NoOpSPI;
//! use mcp25xx::registers::{OperationMode, RXB0CTRL, RXM};
//! use mcp25xx::{CanFrame, Config, MCP25xx};
//!
//! # let spi = NoOpSPI;
//! #
//! // spi is a struct implementing embedded_hal::spi::SpiDevice.
//!
//! let mut mcp25xx = MCP25xx { spi };
//!
//! let config = Config::default()
//!     .mode(OperationMode::NormalOperation)
//!     .bitrate(CNF_500K_BPS)
//!     .receive_buffer_0(RXB0CTRL::default().with_rxm(RXM::ReceiveAny));
//!
//! mcp25xx.apply_config(&config).unwrap();
//!
//! // Send a frame
//! let can_id = StandardId::new(123).unwrap();
//! let data = [1, 2, 3, 4, 5, 6, 7, 8];
//! let frame = CanFrame::new(can_id, &data).unwrap();
//! mcp25xx.transmit(&frame).unwrap();
//!
//! // Receive a frame
//! if let Ok(frame) = mcp25xx.receive() {
//!     let _can_id = frame.id();
//!     let _data = frame.data();
//! }
//! ```

#![no_std]
#![cfg_attr(doc, feature(doc_cfg))]
use core::fmt::Debug;

pub use config::Config;
pub use embedded_can;
use embedded_can::{ErrorKind, Frame};
use embedded_hal::spi::{Operation, SpiDevice};
pub use frame::CanFrame;
pub use idheader::IdHeader;

use crate::registers::*;

/// Preconfigured CNF registers for 8, 16 and 20 Mhz oscillators
pub mod bitrates;
/// Register bitfields
pub mod registers;

mod config;
mod frame;
mod idheader;

/// Either a MCP2510, MCP2515 or MCP25625 CAN controller
///
/// ## Note about MCP2515 and MCP25625
/// These chip revisions offer more efficient commands which the MCP2510 does not support.
/// You can opt in to using these by activating the `mcp2515` or `mcp25625` feature of this crate.
pub struct MCP25xx<SPI> {
    pub spi: SPI,
}

impl<SPI: SpiDevice> MCP25xx<SPI> {
    /// Performs the following steps:
    /// * resets the CAN Controller (this resets all registers and puts it into configuration mode)
    /// * applies configuration
    /// * applies selected operation mode
    ///
    /// ## Note about Masks
    /// The default state of the mask registers is all zeros, which means, filters get ignored.
    /// You should give values for both mask registers even if you only intend to use one receive buffer.
    ///
    /// ```
    /// # use mcp25xx::doctesthelper::get_mcp25xx;
    /// # use mcp25xx::{AcceptanceFilter, Config, MCP25xx};
    /// # use mcp25xx::registers::OperationMode;
    /// # use mcp25xx::bitrates::clock_16mhz::CNF_500K_BPS;
    /// # use embedded_can::StandardId;
    ///
    /// let mut mcp25xx: MCP25xx<_> = get_mcp25xx();
    ///
    /// let can_id = StandardId::new(123).unwrap();
    /// let filters = [
    ///     (AcceptanceFilter::Filter0, can_id.into()),
    ///     (AcceptanceFilter::Mask0, StandardId::MAX.into()),
    ///     (AcceptanceFilter::Mask1, StandardId::MAX.into()),
    /// ];
    ///
    /// let config = Config::default()
    ///     .mode(OperationMode::NormalOperation)
    ///     .bitrate(CNF_500K_BPS)
    ///     .filters(&filters);
    /// mcp25xx.apply_config(&config).unwrap();
    /// ```
    pub fn apply_config(&mut self, config: &Config<'_>) -> Result<(), SPI::Error> {
        self.reset()?;
        self.set_bitrate(config.cnf)?;
        self.write_register(config.rxb0ctrl)?;
        self.write_register(config.rxb1ctrl)?;
        for &(filter, id_header) in config.filters {
            self.set_filter(filter, id_header)?;
        }
        self.write_register(config.canctrl)
    }

    /// Set the controller to NormalOperation, Sleep, Loopback, ListenOnly or Configuration
    pub fn set_mode(&mut self, mode: OperationMode) -> Result<(), SPI::Error> {
        let reg = CANCTRL::new().with_reqop(mode);
        self.modify_register(reg, 0b11100000)
    }

    /// Set clock settings
    ///
    /// See [`bitrates`] for preconfigured settings for different oscillator frequencies.
    ///
    /// ## Note:
    /// The controller needs to be in Configuration Mode for this
    pub fn set_bitrate(&mut self, cnf: CNF) -> Result<(), SPI::Error> {
        self.write_registers(CNF3::ADDRESS, &cnf.into_bytes())
    }

    /// Set individual receive buffer filters or masks
    ///
    /// ## Note:
    /// The controller needs to be in Configuration Mode for this
    ///
    /// ```
    /// # use mcp25xx::doctesthelper::get_mcp25xx;
    /// use embedded_can::{StandardId, ExtendedId};
    /// use mcp25xx::{MCP25xx, IdHeader, AcceptanceFilter::*};
    ///
    /// let mut mcp25xx: MCP25xx<_> = get_mcp25xx();
    ///
    /// let std_id = StandardId::new(1234).unwrap();
    /// let ext_id = ExtendedId::new(4321).unwrap();
    ///
    /// mcp25xx.set_filter(Mask0, IdHeader::from(StandardId::MAX));
    /// mcp25xx.set_filter(Filter0, IdHeader::from(std_id)).unwrap();
    ///
    /// mcp25xx.set_filter(Filter2, IdHeader::from(ext_id)).unwrap();
    /// #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    /// mcp25xx.set_filter(Filter3, IdHeader::with_two_data_bytes(std_id, [4, 5])).unwrap();
    ///
    /// ```
    pub fn set_filter(&mut self, filter: AcceptanceFilter, id: IdHeader) -> Result<(), SPI::Error> {
        self.write_registers(filter as u8, &id.into_bytes())
    }

    /// Read status flags
    pub fn read_status(&mut self) -> Result<ReadStatusResponse, SPI::Error> {
        let mut buf = [0];
        self.spi.transaction(&mut [
            Operation::Write(&[Instruction::ReadStatus as u8]),
            Operation::Read(&mut buf),
        ])?;
        Ok(ReadStatusResponse::from_bytes(buf))
    }

    /// Reset internal registers to the default state. Sets Configuration mode.
    pub fn reset(&mut self) -> Result<(), SPI::Error> {
        self.spi.write(&[Instruction::Reset as u8])
    }

    /// Read receive buffer status flags
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    #[cfg_attr(doc, doc(cfg(any(feature = "mcp2515", feature = "mcp25625"))))]
    pub fn rx_status(&mut self) -> Result<RxStatusResponse, SPI::Error> {
        let mut buf = [0];
        self.spi.transaction(&mut [
            Operation::Write(&[Instruction::RxStatus as u8]),
            Operation::Read(&mut buf),
        ])?;
        Ok(RxStatusResponse::from_bytes(buf))
    }
}

#[derive(Debug)]
pub struct SpiError<E>(pub E);

impl<E: Debug> embedded_can::Error for SpiError<E> {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl<SPI: SpiDevice> embedded_can::nb::Can for MCP25xx<SPI> {
    type Frame = CanFrame;
    type Error = SpiError<SPI::Error>;

    fn transmit(
        &mut self,
        frame: &Self::Frame,
    ) -> nb::Result<Option<Self::Frame>, SpiError<SPI::Error>> {
        let status = self.read_status().map_err(SpiError)?;
        let mut buf_idx = TxBuffer::TXB0;
        if status.txreq0() {
            buf_idx = TxBuffer::TXB1;
            if status.txreq1() {
                buf_idx = TxBuffer::TXB2;
                if status.txreq2() {
                    // TODO replace a pending lower priority frame
                    return Err(nb::Error::WouldBlock);
                }
            }
        }

        self.load_tx_buffer(buf_idx, frame).map_err(SpiError)?;
        self.request_to_send(buf_idx).map_err(SpiError)?;
        Ok(None)
    }

    fn receive(&mut self) -> nb::Result<Self::Frame, SpiError<SPI::Error>> {
        // TODO look at https://www.microchip.com/forums/tm.aspx?m=620741
        let status = self.read_status().map_err(SpiError)?;
        if status.rx0if() {
            Ok(self.read_rx_buffer(RxBuffer::RXB0).map_err(SpiError)?)
        } else if status.rx1if() {
            Ok(self.read_rx_buffer(RxBuffer::RXB1).map_err(SpiError)?)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<SPI: SpiDevice> embedded_can::blocking::Can for MCP25xx<SPI> {
    type Frame = CanFrame;
    type Error = SpiError<SPI::Error>;

    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
        let mut replaced_frame;
        let mut frame_to_transmit = frame;
        while let Some(f) = nb::block!(embedded_can::nb::Can::transmit(self, frame_to_transmit))? {
            replaced_frame = f;
            frame_to_transmit = &replaced_frame;
        }
        Ok(())
    }

    fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
        nb::block!(embedded_can::nb::Can::receive(self))
    }
}

impl<SPI: SpiDevice> MCP25xx<SPI> {
    /// Read a single register
    pub fn read_register<R: Register>(&mut self) -> Result<R, SPI::Error> {
        let mut reg = [0];
        self.spi.transaction(&mut [
            Operation::Write(&[Instruction::Read as u8, R::ADDRESS]),
            Operation::Read(&mut reg),
        ])?;
        Ok(reg[0].into())
    }

    /// Write a single register
    pub fn write_register<R: Register + Into<u8>>(&mut self, reg: R) -> Result<(), SPI::Error> {
        self.spi
            .write(&[Instruction::Write as u8, R::ADDRESS, reg.into()])
    }

    /// Modify a single register
    ///
    /// Only registers implementing [`Modify`] support the `Modify` Instruction
    pub fn modify_register<R: Register + Modify + Into<u8>>(
        &mut self,
        reg: R,
        mask: u8,
    ) -> Result<(), SPI::Error> {
        self.spi
            .write(&[Instruction::BitModify as u8, R::ADDRESS, mask, reg.into()])
    }

    /// Read multiple consecutive registers
    pub fn read_registers(&mut self, start_address: u8, buf: &mut [u8]) -> Result<(), SPI::Error> {
        self.spi.transaction(&mut [
            Operation::Write(&[Instruction::Read as u8, start_address]),
            Operation::Read(buf),
        ])
    }

    /// Write multiple consecutive registers
    pub fn write_registers(&mut self, start_address: u8, data: &[u8]) -> Result<(), SPI::Error> {
        self.spi.transaction(&mut [
            Operation::Write(&[Instruction::Write as u8, start_address]),
            Operation::Write(data),
        ])
    }

    /// Request the selected transmit buffer to send a CAN frame
    pub fn request_to_send(&mut self, buf_idx: TxBuffer) -> Result<(), SPI::Error> {
        self.spi
            .write(&[Instruction::Rts as u8 | (1 << buf_idx as u8)])
    }

    /// Set up the selected transmit buffer with CAN frame data
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    pub fn load_tx_buffer(
        &mut self,
        buf_idx: TxBuffer,
        frame: &CanFrame,
    ) -> Result<(), SPI::Error> {
        let data = &frame.as_bytes()[0..5 + frame.dlc()];

        self.spi.transaction(&mut [
            Operation::Write(&[Instruction::LoadTxBuffer as u8 | (buf_idx as u8 * 2)]),
            Operation::Write(data),
        ])
    }

    /// Set up the selected transmit buffer with CAN frame data
    #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
    #[inline]
    pub fn load_tx_buffer(
        &mut self,
        buf_idx: TxBuffer,
        frame: &CanFrame,
    ) -> Result<(), SPI::Error> {
        let data = &frame.as_bytes()[0..5 + frame.dlc()];
        self.write_registers(0x31 + 0x10 * buf_idx as u8, data)
    }

    /// Read CAN frame data from the selected receive buffer
    pub fn read_rx_buffer(&mut self, buf_idx: RxBuffer) -> Result<CanFrame, SPI::Error> {
        let mut bytes = [0; 13];
        self.read_rx(buf_idx, &mut bytes)?;
        let frame = CanFrame::from_bytes(bytes);

        #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
        // need to manually reset the interrupt flag bit if Instruction::ReadRxBuffer is not available
        self.modify_register(CANINTF::new(), 1 << buf_idx as u8)?;
        Ok(frame)
    }

    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    fn read_rx(&mut self, buf_idx: RxBuffer, bytes: &mut [u8; 13]) -> Result<(), SPI::Error> {
        self.spi.transaction(&mut [
            Operation::Write(&[Instruction::ReadRxBuffer as u8 | (buf_idx as u8 * 2)]),
            Operation::Read(bytes),
        ])
    }

    #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
    fn read_rx(&mut self, buf_idx: RxBuffer, bytes: &mut [u8; 13]) -> Result<(), SPI::Error> {
        self.spi.transaction(&mut [
            Operation::Write(&[Instruction::Read as u8, 0x61 + 0x10 * buf_idx as u8]),
            Operation::Read(bytes),
        ])
    }
}

/// Filters and Masks of the two receive buffers
#[derive(Copy, Clone, Debug)]
pub enum AcceptanceFilter {
    /// Associated with Receive Buffer 0
    Filter0 = 0x00,
    /// Associated with Receive Buffer 0
    Filter1 = 0x04,
    /// Associated with Receive Buffer 1
    Filter2 = 0x08,
    /// Associated with Receive Buffer 1
    Filter3 = 0x10,
    /// Associated with Receive Buffer 1
    Filter4 = 0x14,
    /// Associated with Receive Buffer 1
    Filter5 = 0x18,
    /// Associated with Receive Buffer 0
    Mask0 = 0x20,
    /// Associated with Receive Buffer 1
    Mask1 = 0x24,
}

/// Transmit buffer
#[derive(Copy, Clone, Debug)]
pub enum TxBuffer {
    /// Transmit buffer 0
    TXB0 = 0,
    /// Transmit buffer 1
    TXB1 = 1,
    /// Transmit buffer 2
    TXB2 = 2,
}

/// Receive buffer
#[derive(Copy, Clone, Debug)]
pub enum RxBuffer {
    /// Receive Buffer 0
    RXB0 = 0,
    /// Receive Buffer 1
    RXB1 = 1,
}

/// Instruction supported by the CAN controller
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Instruction {
    /// Resets internal registers to the default state, sets Configuration mode.
    Reset = 0b1100_0000,
    /// Reads data from the register beginning at the selected address.
    Read = 0b0000_0011,
    /// Writes data to the register beginning at the selected address.
    Write = 0b0000_0010,
    /// Instructs the controller to begin the message transmission sequence for
    /// any of the transmit buffers specified in `0b1000_0nnn`.
    Rts = 0b1000_0000,
    /// Quick polling command that reads several Status bits for transmit and receive functions.
    ReadStatus = 0b1010_0000,
    /// Allows the user to set or clear individual bits in a particular register.
    ///
    /// Note: Not all registers can be bit modified with this command.
    /// Executing this command on registers that are not bit modifiable will force the mask to FFh.
    ///
    /// Registers that can be modified with this command implement [`Modify`].
    BitModify = 0b0000_0101,

    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    #[cfg_attr(doc, doc(cfg(any(feature = "mcp2515", feature = "mcp25625"))))]
    /// Quick polling command that indicates a filter match and message type
    /// (standard, extended and/or remote) of the received message.
    RxStatus = 0b1011_0000,
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    #[cfg_attr(doc, doc(cfg(any(feature = "mcp2515", feature = "mcp25625"))))]
    /// When reading a receive buffer, reduces the overhead of a normal `Read`
    /// command by placing the Address Pointer at one of four locations, as
    /// indicated by ‘nm’ in `0b1001_0nm0`.
    ///
    /// Note: The associated RX flag bit (`rxNif` bits in the [`CANINTF`] register) will be cleared after bringing CS high.
    ReadRxBuffer = 0b1001_0000,
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    #[cfg_attr(doc, doc(cfg(any(feature = "mcp2515", feature = "mcp25625"))))]
    /// When loading a transmit buffer, reduces the overhead of a normal `Write`
    /// command by placing the Address Pointer at one of six locations, as
    /// indicated by ‘abc’ in `0b0100_0abc`.
    LoadTxBuffer = 0b0100_0000,
}

#[doc(hidden)]
// FIXME: #[cfg(doctest)] once https://github.com/rust-lang/rust/issues/67295 is fixed.
pub mod doctesthelper;
