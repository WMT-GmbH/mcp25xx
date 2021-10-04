use crate::registers::{CANCTRL, CNF, REQOP, RXB0CTRL, RXB1CTRL};
use crate::{IdHeader, ReceiveBufferFilter};

pub struct Config<'a> {
    pub canctrl: CANCTRL,
    pub cnf: CNF,
    pub rxb0ctrl: RXB0CTRL,
    pub rxb1ctrl: RXB1CTRL,
    pub filters: &'a [(ReceiveBufferFilter, IdHeader)],
}

impl<'a> Config<'a> {
    pub fn mode(&mut self, mode: REQOP) -> &mut Self {
        self.canctrl.set_reqop(mode);
        self
    }
    pub fn can_control_register(&mut self, canctrl: CANCTRL) -> &mut Self {
        self.canctrl = canctrl;
        self
    }
    pub fn bitrate(&mut self, cnf: CNF) -> &mut Self {
        self.cnf = cnf;
        self
    }
    pub fn receive_buffer_0(&mut self, rxb0ctrl: RXB0CTRL) -> &mut Self {
        self.rxb0ctrl = rxb0ctrl;
        self
    }
    pub fn receive_buffer_1(&mut self, rxb1ctrl: RXB1CTRL) -> &mut Self {
        self.rxb1ctrl = rxb1ctrl;
        self
    }
    pub fn filters(&mut self, filters: &'a [(ReceiveBufferFilter, IdHeader)]) -> &mut Self {
        self.filters = filters;
        self
    }
}
