use core::arch::asm;

pub mod NR;
pub mod err;
pub mod file;


/// Performs a linux syscall with three arguments.
///
/// # Safety
/// The user must check that the memory accessed/written by the syscall is conforming
/// to the 
pub unsafe fn syscall3(nr: NR::Nr, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    let nr = nr as u64;
    let ret: u64;
    let ret2: u64;
    // SAFETY: using the asm! per the
    // [Rules for inline assembly in The Rust Reference](https://doc.rust-lang.org/nightly/reference/inline-assembly.html#rules-for-inline-assembly)
    // Given:
    //  1. syscalls have a list of registers as parameters, thus they should not care
    //    about other unused registers.
    //  2. syscalls leave registers
    //  3. I BELIEVE syscalls cannot unwind.
    //  4. syscalls restore the RFLAGS register, except for the RF flag
    //      that is cleared.
    //  5. Doesn't use the stack. 
    // Provides:
    // 1. Any registers not specified as inputs will contain an undefined value
    //    on entry to the asm block.
    //      Is irrellevant per G1.
    // 2. Any registers not specified as outputs must have the same value upon
    //    exiting the asm block as they had on entry, otherwise behavior is undefined.
    //      Is irrelevant per G2.
    // 3. Behavior is undefined if execution unwinds out of an asm block.
    //      Is irrelevant per G3.
    // 4. The set of memory locations that assembly code is allowed to read
    //    and write are the same as those allowed for an FFI function. 
    //      Other than the registers, this is specific to each syscall.
    // 5. Unless the `nostack` option is set, asm code is allowed
    //    to use stack space below the stack pointer.
    //      It is not the case, but could be considered.
    // 6. Unless the nostack option is set, asm code is allowed
    //    to use stack space below the stack pointer.
    //      It is not the case.
    // 7. If the pure option is set then behavior is undefined if the asm! has
    //    side-effects other than its direct outputs. Behavior is also
    //    undefined if two executions of the asm! code with the same
    //    inputs result in different outputs.
    //      It is not the case.
    // 8. These flags registers must be restored upon exiting the asm block if
    //    the `preserves_flags` option is set
    //      It is not the case, but could be considered.
    // 9. On x86, the direction flag (DF in EFLAGS) is clear on entry to an asm
    //    block and must be clear on exit.
    //      The 
    // Sources:
    // 1. The linux entry point of syscalls: https://github.com/torvalds/linux/blob/4652b8e4f3ffa48c706ec334f048c217a7d9750d/arch/x86/entry/entry_64.S#L49-L85
    // 2. man 2 syscall
    // 3. The Intel® 64 and IA-32 Architectures: Software Developer’s Manual
    asm!("syscall",
     in("rax") nr,
     in("rdi") arg1,
     in("rsi") arg2,
     in("rdx") arg3,
     lateout("rax") ret,
     lateout("rcx") _,
     options(nostack, raw),
    );
    ret
}