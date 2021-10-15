use core::convert::Infallible;

use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::OutputPin;

use crate::{MCP25xx, SpiWithCs};

/// used for docs tests
pub fn get_mcp25xx() -> MCP25xx<SpiWithCs<NoOpSPI, NoOpCS>> {
    MCP25xx(SpiWithCs {
        spi: NoOpSPI,
        cs: NoOpCS,
    })
}

pub struct NoOpCS;
pub struct NoOpSPI;

impl OutputPin for NoOpCS {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Write<u8> for NoOpSPI {
    type Error = Infallible;

    fn write(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Transfer<u8> for NoOpSPI {
    type Error = Infallible;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        Ok(words)
    }
}
