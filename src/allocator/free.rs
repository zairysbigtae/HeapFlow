use colored::Colorize;

use super::{Allocator, BlockHeader, MAGIC_ALLOCATED, MAGIC_FREE};

impl Allocator {
    pub fn free(&self, ptr: *mut u8, debug: bool) {
        if ptr.is_null() {
            panic!("{} Tried to free a null pointer. Exiting..",
                "[ERROR]".red()
            );
            return;
        }

        let header_ptr = unsafe { (ptr as *mut BlockHeader).offset(-1) };
        let header_ref = unsafe { &mut *header_ptr };

        if debug {
            println!("{} Freeing block at {:p}: magic=0x{:x} used={} size={}",
                "[FREEING]".green(),
                header_ptr as *const _,
                header_ref.magic,
                header_ref.used,
                header_ref.size,
            );
        }

        if header_ref.magic != MAGIC_ALLOCATED 
        || !header_ref.used {
            panic!("{} Trying to free an invalid or already freed block!", 
                "[ERROR]".red()
            );
            return;
        }

        // mark this block as free and unused
        header_ref.magic = MAGIC_FREE;
        header_ref.used = false;

        if debug {
            println!("{} Block freed at {:p}",
                "[FREEING]".green(),
                header_ptr
            );
        }
    }
}
