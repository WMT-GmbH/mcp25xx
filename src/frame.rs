use embedded_can::Id;

use crate::registers::DLC;
use crate::IdHeader;

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CanFrame {
    pub(crate) id_header: IdHeader,
    pub(crate) dlc: DLC,
    pub(crate) data: [u8; 8],
}

impl CanFrame {
    pub fn as_bytes(&self) -> &[u8; 13] {
        // SAFETY:
        // Frame is [repr(C)] without any padding bytes
        unsafe { &*(self as *const CanFrame as *const [u8; core::mem::size_of::<CanFrame>()]) }
    }

    pub fn from_bytes(bytes: [u8; 13]) -> CanFrame {
        // SAFETY:
        // Frame is [repr(C)] without any padding bytes
        // Also, there are no invariants that the bytes that compose Frame must uphold
        let mut frame: CanFrame = unsafe { core::mem::transmute(bytes) };
        if frame.dlc.dlc() > 8 {
            frame.dlc.set_dlc(8);
        }
        frame
    }
}

impl embedded_can::Frame for CanFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Result<Self, ()> {
        if data.len() > 8 {
            return Err(());
        }

        let mut frame = CanFrame {
            id_header: IdHeader::from(id.into()),
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
        Ok(CanFrame {
            id_header: IdHeader::from(id.into()),
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
