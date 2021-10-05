use embedded_can::{ExtendedId, Id, StandardId};

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct IdHeader {
    sidh: u8,
    sidl: u8,
    eid8: u8,
    eid0: u8,
}

impl IdHeader {
    #[cfg(any(feature = "mcp2515", feature = "mcp25625"))]
    pub fn with_two_data_bytes(id: StandardId, bytes: [u8; 2]) -> Self {
        let id = id.as_raw();
        IdHeader {
            sidh: (id >> 3) as u8,
            sidl: (id as u8 & 0b0000_0111) << 5,
            eid8: bytes[0],
            eid0: bytes[1],
        }
    }

    pub fn id(&self) -> Id {
        if self.exide() {
            let id = ((self.sidh as u32) << 21)
                | ((self.sidl as u32 & 0xE0) << 13)
                | (((self.sidl & 0b11) as u32) << 16)
                | ((self.eid8 as u32) << 8)
                | self.eid0 as u32;
            // SAFETY:
            // arithmetic above is always in bounds
            unsafe { Id::Extended(embedded_can::ExtendedId::new_unchecked(id)) }
        } else {
            let id = ((self.sidh as u16) << 3) | ((self.sidl as u16) >> 5);
            // SAFETY:
            // arithmetic above is always in bounds
            unsafe { Id::Standard(embedded_can::StandardId::new_unchecked(id)) }
        }
    }

    #[inline]
    pub(crate) fn exide(&self) -> bool {
        self.sidl & 0b0000_1000 > 0
    }

    pub(crate) fn into_bytes(self) -> [u8; 4] {
        [self.sidh, self.sidl, self.eid8, self.sidh]
    }
}

impl From<Id> for IdHeader {
    fn from(id: Id) -> Self {
        match id {
            Id::Standard(id) => id.into(),
            Id::Extended(id) => id.into(),
        }
    }
}
impl From<StandardId> for IdHeader {
    #[inline]
    fn from(id: StandardId) -> Self {
        let id = id.as_raw();
        IdHeader {
            sidh: (id >> 3) as u8,
            sidl: (id as u8 & 0b0000_0111) << 5,
            eid8: 0,
            eid0: 0,
        }
    }
}
impl From<ExtendedId> for IdHeader {
    #[inline]
    fn from(id: ExtendedId) -> Self {
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

impl core::fmt::Debug for IdHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IdHeader").field("id", &self.id()).finish()
    }
}
