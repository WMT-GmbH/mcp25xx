use embedded_hal_mock::spi::Transaction;

use mcp25xx::registers::*;
use mcp25xx::{CanFrame, Instruction};

use crate::mock_bus::get_mock_bus;
use embedded_can::{Can, Frame, Id, StandardId};

mod mock_bus;

#[test]
fn test_set_mode() {
    let mut bus = get_mock_bus(
        &[Transaction::write(vec![
            Instruction::BitModify as u8,
            CANCTRL::ADDRESS,
            0b11100000,
            0b10000000,
        ])],
        1,
    );
    bus.set_mode(OperationMode::Configuration).unwrap();
    bus.cs.done();
    bus.spi.done();
}

#[test]
fn test_set_bitrate() {
    let mut bus = get_mock_bus(
        &[
            Transaction::write(vec![Instruction::Write as u8, CNF3::ADDRESS]),
            Transaction::write(vec![0x82, 0x90, 0x00]),
        ],
        1,
    );

    bus.set_bitrate(mcp25xx::bitrates::clock_8mhz::CNF_500K_BPS)
        .unwrap();
    bus.cs.done();
    bus.spi.done();
}

#[test]
fn test_transmit() {
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    let load_instruction = vec![Instruction::LoadTxBuffer as u8];
    #[cfg(not(any(feature = "mcp2515", feature = "mcp25625")))]
    let load_instruction = vec![Instruction::Write as u8, 0x31];

    let mut bus = get_mock_bus(
        &[
            Transaction::write(vec![Instruction::ReadStatus as u8]),
            Transaction::transfer(vec![0], vec![0]),
            Transaction::write(load_instruction),
            Transaction::write(vec![0, 32, 0, 0, 3, 1, 2, 3]),
            Transaction::write(vec![Instruction::Rts as u8 | 1]),
        ],
        3,
    );

    let frame = CanFrame::new(Id::Standard(StandardId::new(1).unwrap()), &[1, 2, 3]).unwrap();

    bus.try_transmit(&frame).unwrap();
    bus.cs.done();
    bus.spi.done();
}
