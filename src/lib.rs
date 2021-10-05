#![no_std]

use core::convert::Infallible;
use core::fmt::Debug;

use embedded_can::Frame;
use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::OutputPin;

pub use config::Config;
pub use frame::CanFrame;
pub use idheader::IdHeader;

use crate::registers::*;

pub mod bitrates;
mod config;
mod frame;
mod idheader;
pub mod registers;

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

pub struct MCP25xx<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    pub spi: SPI,
    pub cs: CS,
}

impl<SPI, CS> MCP25xx<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    /// ```
    /// # use mcp25xx::doctesthelper::get_mcp25xx;
    /// use mcp25xx::{MCP25xx, Config};
    /// use mcp25xx::registers::{REQOP, RXB0CTRL, RXM};
    /// use mcp25xx::bitrates::clock_16mhz::CNF_500K_BPS;
    ///
    /// let mut mcp25xx: MCP25xx<_,_> = get_mcp25xx();
    /// let config = Config::default()
    ///     .mode(REQOP::NormalOperation)
    ///     .bitrate(CNF_500K_BPS)
    ///     .receive_buffer_0(RXB0CTRL::default().with_rxm(RXM::ReceiveAny));
    /// mcp25xx.apply_config(&config).unwrap();
    /// ```
    pub fn apply_config(
        &mut self,
        config: &Config<'_>,
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.reset()?;
        self.set_bitrate(config.cnf)?;
        self.write_register(config.rxb0ctrl)?;
        self.write_register(config.rxb1ctrl)?;
        for &(filter, id_header) in config.filters {
            self.set_filter(filter, id_header)?;
        }
        self.write_register(config.canctrl)
    }

    pub fn set_mode(&mut self, mode: REQOP) -> Result<(), <SPI as Transfer<u8>>::Error> {
        let reg = CANCTRL::new().with_reqop(mode);
        self.modify_register(reg, 0b11100000)
    }

    pub fn set_bitrate(&mut self, cnf: CNF) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.write_registers(CNF3::ADDRESS, &cnf.into_bytes())
    }
    /// Note: Requires Configuration mode
    ///
    /// ```
    /// # use mcp25xx::doctesthelper::get_mcp25xx;
    /// use embedded_can::{StandardId, ExtendedId};
    /// use mcp25xx::{MCP25xx, IdHeader, AcceptanceFilter::*};
    ///
    /// let mut mcp25xx: MCP25xx<_,_> = get_mcp25xx();
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
    pub fn set_filter(
        &mut self,
        filter: AcceptanceFilter,
        id: IdHeader,
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.write_registers(filter as u8, &id.into_bytes())
    }

    pub fn read_status(&mut self) -> Result<ReadStatusResponse, <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi.write(&[Instruction::ReadStatus as u8])?;
        let mut buf = [0];
        self.spi.transfer(&mut buf)?;
        self.cs.set_high().ok();
        Ok(ReadStatusResponse::from_bytes(buf))
    }

    /// Resets internal registers to the default state, sets Configuration mode.
    pub fn reset(&mut self) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi.write(&[Instruction::Reset as u8])?;
        self.cs.set_high().ok();
        Ok(())
    }

    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    pub fn rx_status(&mut self) -> Result<RxStatusResponse, <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi.write(&[Instruction::RxStatus as u8])?;
        let mut buf = [0];
        self.spi.transfer(&mut buf)?;
        self.cs.set_high().ok();
        Ok(RxStatusResponse::from_bytes(buf))
    }
}

impl<SPI, CS> embedded_can::Can for MCP25xx<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    type Frame = crate::frame::CanFrame;
    type Error = <SPI as Transfer<u8>>::Error;

    fn try_transmit(
        &mut self,
        frame: &Self::Frame,
    ) -> nb::Result<Option<Self::Frame>, Self::Error> {
        let status = self.read_status()?;
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

        let registers = &frame.as_bytes()[0..5 + frame.dlc()];
        self.load_tx_buffer(buf_idx, registers)?;
        self.request_to_send(buf_idx)?;
        Ok(None)
    }

    fn try_receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        // TODO look at https://www.microchip.com/forums/tm.aspx?m=620741
        let status = self.read_status()?;
        if status.rx0if() {
            Ok(self.read_rx_buffer(RxBuffer::RXB0)?)
        } else if status.rx1if() {
            Ok(self.read_rx_buffer(RxBuffer::RXB1)?)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<SPI, CS> MCP25xx<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    pub fn read_register<R: Register>(&mut self) -> Result<R, <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi.write(&[Instruction::Read as u8, R::ADDRESS])?;
        let mut reg = [0];
        self.spi.transfer(&mut reg)?;
        self.cs.set_high().ok();
        Ok(reg[0].into())
    }

    pub fn write_register<R: Register + Into<u8>>(
        &mut self,
        reg: R,
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi
            .write(&[Instruction::Write as u8, R::ADDRESS, reg.into()])?;
        self.cs.set_high().ok();
        Ok(())
    }

    pub fn modify_register<R: Register + Modify + Into<u8>>(
        &mut self,
        reg: R,
        mask: u8,
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi
            .write(&[Instruction::BitModify as u8, R::ADDRESS, mask, reg.into()])?;
        self.cs.set_high().ok();
        Ok(())
    }

    pub fn read_registers(
        &mut self,
        start_address: u8,
        buf: &mut [u8],
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi.write(&[Instruction::Read as u8, start_address])?;
        self.spi.transfer(buf)?;
        self.cs.set_high().ok();
        Ok(())
    }

    pub fn write_registers(
        &mut self,
        start_address: u8,
        data: &[u8],
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi.write(&[Instruction::Write as u8, start_address])?;
        self.spi.write(data)?;
        self.cs.set_high().ok();
        Ok(())
    }

    pub fn request_to_send(
        &mut self,
        buf_idx: TxBuffer,
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi
            .write(&[Instruction::Rts as u8 | (1 << buf_idx as u8)])?;
        self.cs.set_high().ok();
        Ok(())
    }

    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    pub fn load_tx_buffer(
        &mut self,
        buf_idx: TxBuffer,
        data: &[u8],
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi
            .write(&[Instruction::LoadTxBuffer as u8 | (buf_idx as u8 * 2)])?;
        self.spi.write(data)?;
        self.cs.set_high().ok();
        Ok(())
    }

    #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
    #[inline]
    pub fn load_tx_buffer(
        &mut self,
        buf_idx: TxBuffer,
        data: &[u8],
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.write_registers(0x31 + 0x10 * buf_idx as u8, data)
    }

    pub fn read_rx_buffer(
        &mut self,
        buf_idx: RxBuffer,
    ) -> Result<crate::frame::CanFrame, <SPI as Transfer<u8>>::Error> {
        // gets a view into the first 5 bytes of Frame
        fn id_bytes(frame: &mut crate::frame::CanFrame) -> &mut [u8; 5] {
            // SAFETY:
            // Frame is [repr(C)] without any padding bytes
            // All bit patterns are valid
            unsafe { &mut *(frame as *mut crate::frame::CanFrame as *mut [u8; 5]) }
        }

        let mut frame = crate::frame::CanFrame::default();

        self.cs.set_low().ok();

        self.send_read_rx_instruction(buf_idx)?;
        self.spi.transfer(id_bytes(&mut frame))?;
        let mut dlc = frame.dlc();
        if dlc > 8 {
            dlc = 8;
            frame.dlc.set_dlc(8);
        }
        self.spi.transfer(&mut frame.data[0..dlc])?;

        self.cs.set_high().ok();

        #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
        // need to manually reset the interrupt flag bit if Instruction::ReadRxBuffer is not available
        self.modify_register(CANINTF::new(), buf_idx as u8)?;
        Ok(frame)
    }

    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    fn send_read_rx_instruction(
        &mut self,
        buf_idx: RxBuffer,
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.spi
            .write(&[Instruction::ReadRxBuffer as u8 | (buf_idx as u8 * 2)])
    }

    #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
    fn send_read_rx_instruction(
        &mut self,
        buf_idx: RxBuffer,
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.spi
            .write(&[Instruction::Read as u8, 0x61 + 0x10 * buf_idx as u8])
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TxBuffer {
    TXB0 = 0,
    TXB1 = 1,
    TXB2 = 2,
}

#[derive(Copy, Clone, Debug)]
pub enum RxBuffer {
    RXB0 = 0,
    RXB1 = 1,
}

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
    /// Quick polling command that indicates a filter match and message type
    /// (standard, extended and/or remote) of the received message.
    RxStatus = 0b1011_0000,
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    /// When reading a receive buffer, reduces the overhead of a normal `Read`
    /// command by placing the Address Pointer at one of four locations, as
    /// indicated by ‘nm’ in `0b1001_0nm0`.
    ///
    /// Note: The associated RX flag bit (`rxNif` bits in the [`CANINTF`] register) will be cleared after bringing CS high.
    ReadRxBuffer = 0b1001_0000,
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    /// When loading a transmit buffer, reduces the overhead of a normal `Write`
    /// command by placing the Address Pointer at one of six locations, as
    /// indicated by ‘abc’ in `0b0100_0abc`.
    LoadTxBuffer = 0b0100_0000,
}

#[doc(hidden)]
// FIXME: #[cfg(doctest)] once https://github.com/rust-lang/rust/issues/67295 is fixed.
pub mod doctesthelper;
