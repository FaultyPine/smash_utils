use skyline::hooks::{getRegionAddress, Region};
use std::ptr::null;

pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub unsafe fn get_text() -> u64 {
    getRegionAddress(Region::Text) as u64
}

#[inline(always)]
pub fn dump_trace() {
    unsafe {
        let txt = getRegionAddress(Region::Text) as u64;
        println!("Current txt: {:#x}", txt);

        let mut lr = get_lr();
        let mut fp = get_fp();

        println!("Current LR: {:#x}", lr as u64);

        while fp != null() {
            lr = *fp.offset(1) as *const u64;
            if lr != null() {
                println!("LR: {:#x}", (lr as u64) - txt);
            }
            fp = *fp as *const u64;
        }
    }
}

#[inline(always)]
fn get_lr() -> *const u64 {
    let r;
    unsafe {
        llvm_asm!("mov $0, x30" : "=r"(r) : : : "volatile")
    }
    r
}

#[inline(always)]
fn get_fp() -> *const u64 {
    let r;
    unsafe {
        llvm_asm!("mov $0, x29" : "=r"(r) : : : "volatile")
    }
    r
}