use core::convert::Infallible;
use core::fmt::Debug;

use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::OutputPin;

pub trait Interface {
    type Error: Debug;
    fn set_cs(&mut self, high: bool);
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error>;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error>;
}

pub struct SpiWithCs<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    pub spi: SPI,
    pub cs: CS,
}

impl<SPI, CS> Interface for SpiWithCs<SPI, CS>
where
    SPI: Transfer<u8>,
    SPI: Write<u8, Error = <SPI as Transfer<u8>>::Error>,
    <SPI as Transfer<u8>>::Error: Debug,
    CS: OutputPin<Error = Infallible>,
{
    type Error = <SPI as Transfer<u8>>::Error;

    #[inline]
    fn set_cs(&mut self, high: bool) {
        match high {
            false => self.cs.set_low().ok(),
            true => self.cs.set_high().ok(),
        };
    }

    #[inline]
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(words)
    }

    #[inline]
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.spi.transfer(words)
    }
}
