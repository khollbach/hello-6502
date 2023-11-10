use core::ptr;

fn main() {
    for a in 0x400..0x800 {
        unsafe { write(a, 0x01); }
    }
}

unsafe fn write(addr: u16, val: u16) {
    ptr::write_volatile(addr as *mut u16, val);
}
