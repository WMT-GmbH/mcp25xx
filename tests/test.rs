use embedded_hal_mock::eh1::spi::{Mock, Transaction};

use mcp25xx::registers::*;
use mcp25xx::{CanFrame, Instruction, MCP25xx};

use embedded_can::nb::Can;
use embedded_can::{Frame, Id, StandardId};

#[test]
fn test_set_mode() {
    let bus = Mock::new(&[
        Transaction::transaction_start(),
        Transaction::write_vec(vec![
            Instruction::BitModify as u8,
            CANCTRL::ADDRESS,
            0b11100000,
            0b10000000,
        ]),
        Transaction::transaction_end(),
    ]);
    let mut mock = MCP25xx { spi: bus };
    mock.set_mode(OperationMode::Configuration).unwrap();
    mock.spi.done();
}

#[test]
fn test_set_bitrate() {
    let bus = Mock::new(&[
        Transaction::transaction_start(),
        Transaction::write_vec(vec![Instruction::Write as u8, CNF3::ADDRESS]),
        Transaction::write_vec(vec![0x82, 0x90, 0x00]),
        Transaction::transaction_end(),
    ]);
    let mut mock = MCP25xx { spi: bus };

    mock.set_bitrate(mcp25xx::bitrates::clock_8mhz::CNF_500K_BPS)
        .unwrap();
    mock.spi.done();
}

#[test]
fn test_transmit() {
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    let load_instruction = vec![Instruction::LoadTxBuffer as u8];
    #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
    let load_instruction = vec![Instruction::Write as u8, 0x31];

    let bus = Mock::new(&[
        Transaction::transaction_start(),
        Transaction::write_vec(vec![Instruction::ReadStatus as u8]),
        Transaction::read_vec(vec![0]),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write_vec(load_instruction),
        Transaction::write_vec(vec![0, 32, 0, 0, 3, 1, 2, 3]),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write_vec(vec![Instruction::Rts as u8 | 1]),
        Transaction::transaction_end(),
    ]);
    let mut mock = MCP25xx { spi: bus };

    let frame = CanFrame::new(Id::Standard(StandardId::new(1).unwrap()), &[1, 2, 3]).unwrap();

    mock.transmit(&frame).unwrap();
    mock.spi.done();
}
