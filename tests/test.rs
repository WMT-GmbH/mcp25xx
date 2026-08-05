use embedded_hal_mock::eh1::spi::{Mock, Transaction};

use mcp25xx::registers::*;
use mcp25xx::{CanFrame, Instruction, MCP25xx};
#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
use mcp25xx::RxBuffer;

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

/// The "Read RX Buffer" fast instruction's buffer-select bit ('n' in
/// `0b1001_0nm0`) sits at bit position 2, not 1 — reading RXB1 must send
/// `0x94`, not `0x92` (which is actually "RXB0, start at D0"). Confirmed on
/// real MCP25625 hardware that the previous `buf_idx * 2` encoding
/// silently returned garbage (RXB0's data bytes reinterpreted as a full
/// frame) for any message that landed in RXB1.
#[test]
#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
fn test_read_rx_buffer_rxb0_uses_the_correct_fast_instruction() {
    let bus = Mock::new(&[
        Transaction::transaction_start(),
        Transaction::write_vec(vec![Instruction::ReadRxBuffer as u8]), // 0x90: RXB0, start at SIDH
        Transaction::read_vec(vec![0; 13]),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write_vec(vec![Instruction::BitModify as u8, CANINTF::ADDRESS, 0b0000_0001, 0]),
        Transaction::transaction_end(),
    ]);
    let mut mock = MCP25xx { spi: bus };
    mock.read_rx_buffer(RxBuffer::RXB0).unwrap();
    mock.spi.done();
}

#[test]
#[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
fn test_read_rx_buffer_rxb1_uses_the_correct_fast_instruction() {
    let bus = Mock::new(&[
        Transaction::transaction_start(),
        Transaction::write_vec(vec![Instruction::ReadRxBuffer as u8 | 0b0000_0100]), // 0x94: RXB1, start at SIDH
        Transaction::read_vec(vec![0; 13]),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write_vec(vec![Instruction::BitModify as u8, CANINTF::ADDRESS, 0b0000_0010, 0]),
        Transaction::transaction_end(),
    ]);
    let mut mock = MCP25xx { spi: bus };
    mock.read_rx_buffer(RxBuffer::RXB1).unwrap();
    mock.spi.done();
}
