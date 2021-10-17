use core::convert::Infallible;
use core::fmt::Debug;

use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::OutputPin;

use crate::MCP25xx;

pub trait SpiWithCs {
    type Error: Debug;
    fn set_cs_high(&mut self);
    fn set_cs_low(&mut self);
    fn spi_write(&mut self, words: &[u8]) -> Result<(), Self::Error>;
    fn spi_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error>;
}

impl<SPI, CS> SpiWithCs for MCP25xx<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    type Error = <SPI as Transfer<u8>>::Error;

    #[inline]
    fn set_cs_high(&mut self) {
        self.cs.set_high().ok();
    }
    #[inline]
    fn set_cs_low(&mut self) {
        self.cs.set_low().ok();
    }

    #[inline]
    fn spi_write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(words)
    }

    #[inline]
    fn spi_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.spi.transfer(words)
    }
}
