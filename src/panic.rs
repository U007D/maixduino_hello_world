use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

// emits breakpoint-settable `rust_begin_unwind` symbol
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
