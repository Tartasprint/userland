use core::panic::PanicInfo;

#[panic_handler]
fn panic(_arg: &PanicInfo) -> ! {
    loop {}
}
