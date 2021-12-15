use mcp25xx::bitrates::*;
use mcp25xx::registers::CNF;

const CNF8: [CNF; 13] = [
    // clock_8mhz::CNF_1000K_BPS,
    clock_8mhz::CNF_500K_BPS,
    clock_8mhz::CNF_250K_BPS,
    clock_8mhz::CNF_200K_BPS,
    clock_8mhz::CNF_125K_BPS,
    clock_8mhz::CNF_100K_BPS,
    clock_8mhz::CNF_80K_BPS,
    clock_8mhz::CNF_50K_BPS,
    clock_8mhz::CNF_40K_BPS,
    clock_8mhz::CNF_33K3_BPS,
    clock_8mhz::CNF_31K25_BPS,
    clock_8mhz::CNF_20K_BPS,
    clock_8mhz::CNF_10K_BPS,
    clock_8mhz::CNF_5K_BPS,
];
const CNF16: [CNF; 14] = [
    clock_16mhz::CNF_1000K_BPS,
    clock_16mhz::CNF_500K_BPS,
    clock_16mhz::CNF_250K_BPS,
    clock_16mhz::CNF_200K_BPS,
    clock_16mhz::CNF_125K_BPS,
    clock_16mhz::CNF_100K_BPS,
    clock_16mhz::CNF_80K_BPS,
    clock_16mhz::CNF_83K3_BPS,
    clock_16mhz::CNF_50K_BPS,
    clock_16mhz::CNF_40K_BPS,
    clock_16mhz::CNF_33K3_BPS,
    clock_16mhz::CNF_20K_BPS,
    clock_16mhz::CNF_10K_BPS,
    clock_16mhz::CNF_5K_BPS,
];
const CNF20: [CNF; 11] = [
    clock_20mhz::CNF_1000K_BPS,
    clock_20mhz::CNF_500K_BPS,
    clock_20mhz::CNF_250K_BPS,
    clock_20mhz::CNF_200K_BPS,
    clock_20mhz::CNF_125K_BPS,
    clock_20mhz::CNF_100K_BPS,
    clock_20mhz::CNF_83K3_BPS,
    clock_20mhz::CNF_80K_BPS,
    clock_20mhz::CNF_50K_BPS,
    clock_20mhz::CNF_40K_BPS,
    clock_20mhz::CNF_33K3_BPS,
];

#[test]
fn test_print_bitrate() {
    for cnf in CNF8 {
        dbg!(cnf);
    }
    for cnf in CNF16 {
        dbg!(cnf);
    }
    for cnf in CNF20 {
        dbg!(cnf);
    }
}

#[test]
fn test_bitrates() {
    for cnf in CNF8 {
        test_bitrate(8_000_000, cnf);
    }
    for cnf in CNF16 {
        test_bitrate(16_000_000, cnf);
    }
    for cnf in CNF20 {
        test_bitrate(20_000_000, cnf);
    }
}

fn test_bitrate(f: usize, cnf: CNF) {
    let tq = 2.0 * (cnf.cnf1.brp() as f64 + 1.0) / f as f64;
    let prseg = cnf.cnf2.prseg() as usize + 1;
    let phseg1 = cnf.cnf2.phseg1() as usize + 1;
    let phseg2 = cnf.cnf3.phseg2() as usize + 1;
    let sjw = cnf.cnf1.sjw() as usize + 1;
    let tq_num = 1 + prseg + phseg1 + phseg2;

    assert!(phseg1 >= sjw);
    assert!(phseg2 >= sjw);
    dbg!(1.0 / (tq_num as f64 * tq));
}
