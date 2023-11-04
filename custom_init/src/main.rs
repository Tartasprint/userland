#![no_std]
#![no_main]

use cmn::syscall::file::{read, write};
#[allow(unused)]
use rt;

#[export_name = "main"]
pub fn main() {
    let greeting = "Hello there !\n";
    write(1, greeting.as_bytes());
    let mut buf = [0u8; 6];
    loop {
        match read(0, &mut buf) {
            Ok(len) => {
                write(1, "\nReceived\n".as_bytes());
                write(1, &buf[0..(len as usize)]);
            }
            Err(e) => {
                panic!();
            }
        }
    }
}
