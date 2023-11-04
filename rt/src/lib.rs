#![no_std]
#![feature(start)]

mod panic;

#[export_name = "_start"]
pub unsafe extern "C" fn entry() {
    extern "Rust" {
        fn main() -> !;
    }
    unsafe { main() }
}
