use core::mem;
use ctr::{allocator::mappable_alloc, res::CtrResult, svc, Process};

const EXTENDED_MEMORY_GAMES: [u64; 4] = [
    0x0004000000164800,
    0x0004000000175E00,
    0x00040000001B5000,
    0x00040000001B5100,
];

// Is there a better way to know this?
fn is_extended_memory(title_id: u64) -> bool {
    EXTENDED_MEMORY_GAMES
        .iter()
        .any(|extended_memory_title_id| *extended_memory_title_id == title_id)
}

const CODE_VADDR: u32 = 0x100000;
const CODE_MAX_END_VADDR: u32 = 0x4000000;

const HEAP_VADDR: u32 = 0x8000000;
const HEAP_MAX_END_VADDR: u32 = 0x10000000;

const EXTENDED_HEAP_VADDR: u32 = 0x30000000;
const EXTENDED_HEAP_MAX_END_VADDR: u32 = 0x40000000;

pub enum MemRegion {
    Code,
    Heap,
    ExtendedHeap,
}

fn is_between(value: u32, lower: u32, upper: u32) -> bool {
    lower <= value && value <= upper
}

impl MemRegion {
    pub fn from_addr(addr: u32) -> Option<Self> {
        if is_between(addr, CODE_VADDR, CODE_MAX_END_VADDR) {
            return Some(MemRegion::Code);
        }

        if is_between(addr, HEAP_VADDR, HEAP_MAX_END_VADDR) {
            return Some(MemRegion::Heap);
        }

        if is_between(addr, EXTENDED_HEAP_VADDR, EXTENDED_HEAP_MAX_END_VADDR) {
            return Some(MemRegion::ExtendedHeap);
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemPattern {
    offset: usize,
    addr: u32,
}

impl MemPattern {
    pub fn offset(&self) -> usize {
        self.offset
    }
}

/// The game memory points to a physical address that is always available.
#[derive(Debug, Clone, Copy)]
pub struct GameMemRegion {
    pub(super) base_addr: u32,
    pub(super) memory: &'static [u8],
}

impl GameMemRegion {
    pub fn new_heap(title_id: u64) -> CtrResult<Self> {
        let proc = Process::new_from_title_id(title_id)?;

        if is_extended_memory(title_id) {
            return Self::new_extended_memory_heap(proc);
        }

        Self::new_regular(proc, HEAP_VADDR)
    }

    pub fn new_code(title_id: u64) -> CtrResult<Self> {
        let proc = Process::new_from_title_id(title_id)?;
        Self::new_regular(proc, CODE_VADDR)
    }

    fn new_extended_memory_heap(proc: Process) -> CtrResult<Self> {
        let query_response = proc.query_memory(EXTENDED_HEAP_VADDR)?;
        let heap_size = query_response.mem_info.size;
        return Ok(Self {
            base_addr: EXTENDED_HEAP_VADDR,
            // Extended memory mode resets the console, and this is always the physical address.
            memory: unsafe { core::slice::from_raw_parts(0xa0000000 as *mut u8, heap_size) },
        });
    }

    fn new_regular(proc: Process, addr: u32) -> CtrResult<Self> {
        let query_response = proc.query_memory(addr)?;
        let base_addr = query_response.mem_info.base_addr;
        let mem_size = query_response.mem_info.size;

        let dst = mappable_alloc(mem_size);

        unsafe {
            svc::map_memory_ex(proc.handle(), dst, base_addr, mem_size)?;
            let pa = svc::convert_va_to_pa(dst, true);
            let pa = svc::convert_pa_to_uncached_pa(pa)?;
            svc::unmap_memory_ex(proc.handle(), dst, mem_size)?;

            Ok(Self {
                base_addr,
                memory: core::slice::from_raw_parts(pa, mem_size),
            })
        }
    }

    pub fn slice(&self) -> &'static [u8] {
        self.memory
    }

    pub fn slice_mut(&self) -> &'static mut [u8] {
        // This is alright since it's the game's memory.
        // The entire point of the plugin system is reading and writing to the game's memory.
        // For our process, this doesn't affect our own memory.
        #[allow(mutable_transmutes)]
        unsafe {
            mem::transmute::<&'static [u8], &'static mut [u8]>(self.memory)
        }
    }

    // Not efficient, but it will do for now
    pub fn find_pattern(&self, bytes: &[u8]) -> Option<MemPattern> {
        self.memory
            .windows(bytes.len())
            .position(|window| window == bytes)
            .map(|offset| MemPattern {
                offset,
                addr: (offset as u32) + self.base_addr,
            })
    }
}
