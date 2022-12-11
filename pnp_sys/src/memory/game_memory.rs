use super::{safe_slice, GameMemRegion, MemRegion};
use ctr::res::CtrResult;

#[derive(Debug, Clone, Copy)]
pub struct GameMemory {
    code: GameMemRegion,
    heap: GameMemRegion,
}

impl GameMemory {
    pub fn new(title_id: u64) -> CtrResult<Self> {
        Ok(Self {
            code: GameMemRegion::new_code(title_id)?,
            heap: GameMemRegion::new_heap(title_id)?,
        })
    }

    fn region(&self, addr: u32) -> Option<&GameMemRegion> {
        MemRegion::from_addr(addr).map(|region| match region {
            MemRegion::Code => &self.code,
            MemRegion::Heap | MemRegion::ExtendedHeap => &self.heap,
        })
    }

    pub fn read(&self, addr: u32, size: usize) -> Option<&'static [u8]> {
        let region = self.region(addr)?;
        let offset = addr.saturating_sub(region.base_addr) as usize;
        let slice = safe_slice::slice(region.slice(), offset, size);
        Some(slice)
    }

    pub fn write_buf(&self, addr: u32, size: usize) -> Option<&'static mut [u8]> {
        let region = self.region(addr)?;
        let offset = addr.saturating_sub(region.base_addr) as usize;
        let slice = safe_slice::slice_mut(region.slice_mut(), offset, size);
        Some(slice)
    }
}
