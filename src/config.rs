use crate::registers::{OperationMode, CANCTRL, CNF, RXB0CTRL, RXB1CTRL};
use crate::{AcceptanceFilter, IdHeader};

#[derive(Clone, Debug, Default)]
pub struct Config<'a> {
    pub canctrl: CANCTRL,
    pub cnf: CNF,
    pub rxb0ctrl: RXB0CTRL,
    pub rxb1ctrl: RXB1CTRL,
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
}
