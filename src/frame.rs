use crate::registers::DLC;
use core::fmt::{Debug, Formatter};
use embedded_can::Id;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub(crate) struct IdHeader {
    sidh: u8,
    sidl: u8,
    eid8: u8,
    eid0: u8,
}

impl IdHeader {
    fn new(id: Id) -> Self {
        match id {
            Id::Standard(id) => {
                let id = id.as_raw();
                IdHeader {
                    sidh: (id >> 3) as u8,
                    sidl: (id as u8 & 0b0000_0111) << 5,
                    eid8: 0,
                    eid0: 0,
                }
            }
            Id::Extended(id) => {
                let id = id.as_raw();

                IdHeader {
                    sidh: (id >> 21) as u8,
                    sidl: (((id >> 13) & 0b11100000) as u8)
                        | 0b0000_1000
                        | (((id >> 16) & 0b0000_0011) as u8),
                    eid8: (id >> 8) as u8,
                    eid0: id as u8,
                }
            }
        }
    }

    #[inline]
    fn exide(&self) -> bool {
        self.sidl & 0b0000_1000 > 0
    }

    fn id(&self) -> Id {
        if self.exide() {
            todo!()
        } else {
            let id = ((self.sidh as u16) << 3) + ((self.sidl as u16) >> 5);
            // SAFETY:
            // arithmetic above is always in bounds
            unsafe { Id::Standard(embedded_can::StandardId::new_unchecked(id)) }
        }
    }
}

impl Debug for IdHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IdHeader").field("id", &self.id()).finish()
    }
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct Frame {
    pub(crate) id_header: IdHeader,
    pub(crate) dlc: DLC,
    pub(crate) data: [u8; 8],
}

impl Frame {
    pub fn as_bytes(&self) -> &[u8; 13] {
        // SAFETY:
        // Frame is [repr(C)] without any padding bytes
        unsafe { &*(self as *const Frame as *const [u8; core::mem::size_of::<Frame>()]) }
    }

    pub fn from_bytes(bytes: [u8; 13]) -> Frame {
        // SAFETY:
        // Frame is [repr(C)] without any padding bytes
        // Also, there are no invariants that the bytes that compose Frame must uphold
        let mut frame: Frame = unsafe { core::mem::transmute(bytes) };
        if frame.dlc.dlc() > 8 {
            frame.dlc.set_dlc(8);
        }
        frame
    }
}

impl embedded_can::Frame for Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Result<Self, ()> {
        if data.len() > 8 {
            return Err(());
        }

        let mut frame = Frame {
            id_header: IdHeader::new(id.into()),
            dlc: DLC::new().with_dlc(data.len() as u8),
            data: [0; 8],
        };

        frame.data[..data.len()].copy_from_slice(data);

        Ok(frame)
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Result<Self, ()> {
        if dlc > 8 {
            return Err(());
        }
        Ok(Frame {
            id_header: IdHeader::new(id.into()),
            dlc: DLC::new().with_dlc(dlc as u8).with_rtr(true),
            data: [0; 8],
        })
    }

    #[inline]
    fn is_extended(&self) -> bool {
        self.id_header.exide()
    }

    #[inline]
    fn is_remote_frame(&self) -> bool {
        self.dlc.rtr()
    }

    fn id(&self) -> Id {
        self.id_header.id()
    }

    #[inline]
    fn dlc(&self) -> usize {
        self.dlc.dlc() as usize
    }

    #[inline]
    fn data(&self) -> &[u8] {
        &self.data[0..self.dlc()]
    }
}
