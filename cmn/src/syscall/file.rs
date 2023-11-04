use super::{err::E, syscall3, NR::Nr};

/// Attempts to fill `buf` by reading the file pointed by `fd`.
///
/// This is a simple wrapper around the linux `read` system call.
///
/// # Return value
/// On success it returns the number of bytes read into `buf`.
///
/// On error it forwards the error given by the kernel.
/// # Panics
/// If an unknown error is generated by the system call.
pub fn read(fd: u64, buf: &mut [u8]) -> Result<u64, E> {
    // SAFETY: after looking at the documentation
    let ret = unsafe { syscall3(Nr::READ, fd, buf.as_ptr() as u64, buf.len() as u64) } as i64;
    if ret < 0 {
        match E::try_from(-ret as u64) {
            Ok(err) => Err(err),
            Err(()) => panic!(),
        }
    } else {
        Ok(ret as u64)
    }
}

pub fn write(fd: u64, buf: &[u8]) {
    unsafe { syscall3(Nr::WRITE, fd, buf.as_ptr() as u64, buf.len() as u64) };
}
