use std::alloc::{GlobalAlloc, Layout};

use std::os::raw::c_void;

#[link(name = "pthread")]
extern "C" {
    fn hoard_malloc(size: usize) -> *mut c_void;
    fn hoard_posix_memalign(align: usize, size: usize) -> *mut c_void;
    fn hoard_free(ptr: *mut c_void);
}

#[cfg(all(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "mipsel",
              target_arch = "powerpc",
              target_arch = "powerpc64",
              target_arch = "powerpc64le")))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86_64",
              target_arch = "aarch64")))]
const MIN_ALIGN: usize = 16;

pub struct Hoard;

unsafe impl GlobalAlloc for Hoard {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            hoard_malloc(layout.size()) as *mut u8
        } else {
            hoard_posix_memalign(layout.align(), layout.size()) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        hoard_free(ptr as *mut c_void)
    }
}
