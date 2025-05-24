mod free;
use std::sync::Arc;

use colored::Colorize;
use spin::mutex::Mutex;

const MAGIC_ALLOCATED: u32 = 0xC0DECAFE;
const MAGIC_FREE: u32 = 0xDEADC0DE;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockHeader {
    size: usize,
    used: bool,
    next: Option<*mut BlockHeader>,
    magic: u32,
}

pub struct Allocator {
    pub heap: Arc<Mutex<Vec<u8>>>, // Box because its heap allocated yknow? lol
    pub allocations: usize,
}

impl Allocator {
    pub fn new(heap_size: usize) -> Self {
        let heap = Arc::new(Mutex::new(vec![0u8; heap_size]));

        Self {
            heap,
            allocations: 0
        }
    }

    pub fn free_list_allocate(&self, requested_size: usize, debug: bool) -> *mut u8 {
        let mut heap = self.heap.lock();
        let start_of_heap = heap.as_mut_ptr();
        let heap_size = heap.len();
        #[allow(unused_mut)]
        let mut offset = 0;

        while offset < heap_size {
            // basically, the .add(offset) makes it so that it moves from start_of_heap so that
            // theres enough space for our next block
            let header_ptr = unsafe { start_of_heap.add(offset) } as *mut BlockHeader;
            let header_ref = unsafe { &mut *header_ptr };

            if header_ref.magic != MAGIC_ALLOCATED
            && header_ref.magic != MAGIC_FREE {
                if debug {
                    println!("{} Uninitialized block found at {:p}.. Initialiazing a new block at the same location:
                        magic=0x{:x}, used={}, size={}, requested_size={}",
                        "[UNINITIALIZED BLOCK FOUND]".green(),
                        header_ref as *const _,
                        header_ref.magic,
                        header_ref.used,
                        header_ref.size,
                        requested_size
                    );
                }
                // uninitialized block.. make a new one
                header_ref.magic = MAGIC_ALLOCATED;
                header_ref.size = requested_size;
                header_ref.used = true;

                if debug {
                    println!("{} After initiliazation at {:p}:
                        magic=0x{:x}, used{}, size={}, requested_size={}",
                        "[UNINITIALIZED BLOCK FOUND]".green(),
                        header_ref as *const _,
                        header_ref.magic,
                        header_ref.used,
                        header_ref.size,
                        requested_size
                    );
                }

                return unsafe { header_ptr.add(1) } as *mut u8;
            }

            if header_ref.magic == MAGIC_FREE
            && !header_ref.used
            && header_ref.size >= requested_size {
                if debug {
                    println!("{} Block found at {:p}: magic=0x{:x}, used={}, size={}, requested_size={}",
                        "[SUITABLE BLOCK FOUND]".green(),
                        header_ref as *const _,
                        header_ref.magic,
                        header_ref.used,
                        header_ref.size,
                        requested_size,
                    );
                }
                header_ref.magic = MAGIC_ALLOCATED;
                header_ref.used = true;

                if debug {
                    println!("{} Variables are now set at {:p}: magic=0x{:x}, used={}, size={}, requested_size={}",
                        "[SUITABLE BLOCK FOUND]".green(),
                        header_ref as *const _,
                        header_ref.magic,
                        header_ref.used,
                        header_ref.size,
                        requested_size,
                    );
                }

                return unsafe { header_ptr.add(1) } as *mut u8;
            }

            offset += core::mem::size_of::<BlockHeader>() + header_ref.size;
        }

        core::ptr::null_mut()
    }
}
