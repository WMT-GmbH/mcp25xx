use crate::MCP25xx;
use core::convert::Infallible;
use embedded_hal::spi::{ErrorType, Operation, SpiDevice};

/// used for doc tests
pub fn get_mcp25xx() -> MCP25xx<NoOpSPI> {
    MCP25xx { spi: NoOpSPI }
}

pub struct NoOpSPI;

impl SpiDevice for NoOpSPI {
    fn transaction(&mut self, _: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl ErrorType for NoOpSPI {
    type Error = Infallible;
}