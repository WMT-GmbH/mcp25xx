#![no_std]

use core::convert::Infallible;
use core::fmt::Debug;

use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::OutputPin;

use crate::registers::*;
use embedded_can::Frame;

pub mod bitrates;
pub mod frame;
pub mod registers;

#[repr(u8)]
pub enum Instruction {
    Reset = 0b1100_0000,
    Read = 0b0000_0011,
    Write = 0b0000_0010,
    Rts = 0b1000_0000, // | 0b 0000 0 tx2 tx1 tx0
    ReadStatus = 0b1010_0000,
    BitModify = 0b0000_0101,

    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    RxStatus = 0b1011_0000,
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    ReadRxBuffer = 0b1001_0000,
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    LoadTxBuffer = 0b0100_0000,
}

pub struct MCP25xx<SPI, CS> {
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
    pub fn set_mode(&mut self, mode: REQOP) -> Result<(), <SPI as Transfer<u8>>::Error> {
        let reg = CANCTRL::new().with_reqop(mode);
        self.modify_register(reg, 0b11100000)
    }
    pub fn set_bitrate(&mut self, cnf: CNF) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.write_registers(CNF::ADDRESS, &cnf.into_bytes())
    }
}

impl<SPI, CS> embedded_can::Can for MCP25xx<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    type Frame = crate::frame::Frame;
    type Error = <SPI as Transfer<u8>>::Error;

    fn try_transmit(
        &mut self,
        frame: &Self::Frame,
    ) -> nb::Result<Option<Self::Frame>, Self::Error> {
        let status = self.read_status()?;
        let mut buf_idx = 0u8;
        if status.txreq0() {
            buf_idx = 1;
            if status.txreq1() {
                buf_idx = 2;
                if status.txreq2() {
                    // TODO replace a pending lower priority frame
                    return Err(nb::Error::WouldBlock);
                }
            }
        }

        let registers = &frame.as_bytes()[0..5 + frame.dlc()];
        self.load_tx(buf_idx, registers)?;
        self.request_to_send(buf_idx)?;
        Ok(None)
    }

    fn try_receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        todo!()
    }
}

impl<SPI, CS> MCP25xx<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    pub fn read_register<R: Register + From<u8>>(
        &mut self,
    ) -> Result<R, <SPI as Transfer<u8>>::Error> {
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

    pub fn request_to_send(&mut self, buf_idx: u8) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi.write(&[Instruction::Rts as u8 | (1 << buf_idx)])?;
        self.cs.set_high().ok();
        Ok(())
    }

    pub fn read_status(&mut self) -> Result<ReadStatusResponse, <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi.write(&[Instruction::ReadStatus as u8])?;
        let mut buf = [0];
        self.spi.transfer(&mut buf)?;
        self.cs.set_high().ok();
        Ok(ReadStatusResponse::from_bytes(buf))
    }

    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    pub fn load_tx(
        &mut self,
        buf_idx: u8,
        data: &[u8],
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.cs.set_low().ok();
        self.spi
            .write(&[Instruction::LoadTxBuffer as u8 | (buf_idx * 2)])?;
        self.spi.write(data)?;
        self.cs.set_high().ok();
        Ok(())
    }

    #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
    #[inline]
    pub fn load_tx(
        &mut self,
        buf_idx: u8,
        data: &[u8],
    ) -> Result<(), <SPI as Transfer<u8>>::Error> {
        self.write_registers(0x31 + 0x10 * buf_idx, data)
    }
}
