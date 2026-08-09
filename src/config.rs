use crate::registers::{CANCTRL, CANINTE, CNF, OperationMode, RXB0CTRL, RXB1CTRL};
use crate::{AcceptanceFilter, IdHeader};

/// Configuration for:
/// * Clock settings
/// * Operation Mode
/// * Receive buffers
/// * Receive buffer filters and masks
/// * Interrupt enables
/// * Other flags inside the CANCTRL, CNF, RXB0CTRL, RXB1CTRL registers
///
/// `caninte` defaults to all-zero, matching the register's post-reset
/// value, so existing callers see no behavior change unless they opt in
/// via [`interrupts`](Config::interrupts). Callers that read incoming
/// frames through an interrupt pin (rather than polling `read_status`)
/// need to set at least `rx0ie`/`rx1ie` here — without it, `CANINTE` stays
/// at its post-reset default (every interrupt source disabled) and the
/// physical INT pin never asserts for a new message.
#[derive(Clone, Debug, Default)]
pub struct Config<'a> {
    pub canctrl: CANCTRL,
    pub cnf: CNF,
    pub rxb0ctrl: RXB0CTRL,
    pub rxb1ctrl: RXB1CTRL,
    pub caninte: CANINTE,
    pub filters: &'a [(AcceptanceFilter, IdHeader)],
}

impl<'a> Config<'a> {
    #[inline]
    pub fn mode(mut self, mode: OperationMode) -> Self {
        self.canctrl.set_reqop(mode);
        self
    }
    #[inline]
    pub fn can_control_register(mut self, canctrl: CANCTRL) -> Self {
        self.canctrl = canctrl;
        self
    }
    #[inline]
    pub fn bitrate(mut self, cnf: CNF) -> Self {
        self.cnf = cnf;
        self
    }
    #[inline]
    pub fn receive_buffer_0(mut self, rxb0ctrl: RXB0CTRL) -> Self {
        self.rxb0ctrl = rxb0ctrl;
        self
    }
    #[inline]
    pub fn receive_buffer_1(mut self, rxb1ctrl: RXB1CTRL) -> Self {
        self.rxb1ctrl = rxb1ctrl;
        self
    }
    #[inline]
    pub fn filters(mut self, filters: &'a [(AcceptanceFilter, IdHeader)]) -> Self {
        self.filters = filters;
        self
    }
    #[inline]
    pub fn interrupts(mut self, caninte: CANINTE) -> Self {
        self.caninte = caninte;
        self
    }
}
