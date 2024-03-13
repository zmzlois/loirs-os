#![no_std]
#![no_main]

use core::panic::PanicInfo;




#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't put prefix at this function when its compiled
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function name `_start` by
    // default
    loop {}
}
