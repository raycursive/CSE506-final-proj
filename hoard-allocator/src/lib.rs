use std::alloc::{GlobalAlloc, Layout};

use std::cmp;
use std::ffi::{c_int, c_void};
use std::ptr;

#[link(name = "hoard", kind = "static")]
extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn posix_memalign(ptr: *mut *mut c_void, align: usize, size: usize) -> c_int;
    fn free(ptr: *mut c_void);
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "mipsel",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "powerpc64le"
)))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86_64", target_arch = "aarch64")))]
const MIN_ALIGN: usize = 16;

pub struct Hoard;

unsafe impl GlobalAlloc for Hoard {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            malloc(layout.size()) as *mut u8
        } else {
            let mut out = std::ptr::null_mut();
            let ret = posix_memalign(&mut out, layout.align(), layout.size());
            if ret != 0 {
                ptr::null_mut()
            } else {
                out as *mut u8
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            realloc(ptr as *mut c_void, new_size) as *mut u8
        } else {
            let new_ptr = self.alloc(layout);
            ptr::copy(ptr, new_ptr, cmp::min(layout.size(), new_size));
            self.dealloc(ptr, layout);
            new_ptr
        }
    }
}
