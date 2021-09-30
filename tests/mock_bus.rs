use embedded_hal::digital::v2::OutputPin;
use embedded_hal_mock::common::Generic;
use embedded_hal_mock::spi::{Mock, Transaction};
use mcp25xx::MCP25xx;
use std::convert::Infallible;

pub fn get_mock_bus(
    spi_expectations: &[Transaction],
    cs_toggles: usize,
) -> MCP25xx<Generic<Transaction>, MockCS> {
    let cs = MockCS::new(cs_toggles);
    let spi = Mock::new(spi_expectations);
    MCP25xx { spi, cs }
}

pub struct MockCS(usize);

impl OutputPin for MockCS {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        if self.0 % 2 == 1 {
            panic!("CS already low")
        }
        self.0 = self.0.checked_sub(1).expect("too many CS flanks");
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        if self.0 % 2 == 0 {
            panic!("CS already high")
        }
        self.0 = self.0.checked_sub(1).expect("too many CS flanks");
        Ok(())
    }
}

impl MockCS {
    pub fn new(toggles: usize) -> Self {
        MockCS(toggles * 2)
    }

    pub fn done(&self) {
        if self.0 != 0 {
            panic!("not enough CS flanks")
        }
    }
}
