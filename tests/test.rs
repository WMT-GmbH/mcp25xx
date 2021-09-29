use embedded_hal_mock::spi::Transaction;

use mcp25xx::registers::*;
use mcp25xx::Instruction;

use crate::mock_bus::get_mock_bus;

mod mock_bus;

#[test]
fn test_set_mode() {
    let mut bus = get_mock_bus(
        &[Transaction::write(vec![
            Instruction::BitModify as u8,
            CANCTRL::ADDRESS,
            0b11100000,
            0b01000000,
        ])],
        1,
    );
    bus.set_mode(REQOP::Loopback).unwrap();
    bus.cs.done();
    bus.spi.done();
}

#[test]
fn test_set_bitrate() {
    let mut bus = get_mock_bus(
        &[
            Transaction::write(vec![Instruction::Write as u8, CNF::ADDRESS]),
            Transaction::write(vec![0x82, 0x90, 0x00]),
        ],
        1,
    );

    bus.set_bitrate(mcp25xx::bitrates::clock_8mhz::CNF_500K_BPS)
        .unwrap();
    bus.cs.done();
    bus.spi.done();
}
