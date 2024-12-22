use core::arch::asm;

#[cfg(feature = "default_panic_handler")]
pub mod default_panic_handler {
    use core::arch::asm;

    /**
     * The default panic handler. This issues a hardware breakpoint (works in melonDS, not in
     * desmume or retail hardware).
     */
    #[panic_handler]
    pub fn _handle_panic_default(_: &core::panic::PanicInfo) -> ! {
        if cfg!(debug_assertions) {
            unsafe {
                asm!("bkpt");
            }
        }

        loop {}
    }
}

// No clue what these functions do, but GNU ld won't link properly if they aren't set under certain
// conditions.
//
// These are weak linkage'd so that if an end-user does have a use for them, they can be easily
// overwritten by a strongly exported symbol.

#[linkage = "weak"]
#[unsafe(no_mangle)]
pub fn __aeabi_unwind_cpp_pr0() {}
#[linkage = "weak"]
#[unsafe(no_mangle)]
pub fn __aeabi_unwind_cpp_pr1() {}
#[linkage = "weak"]
#[unsafe(no_mangle)]
pub fn __aeabi_unwind_cpp_pr2() {}

// This is vastly easier than doing it at the rust level.

/**
 * Gets the end of the ``.text`` section from the linker.
 */
pub fn get_text_end() -> *mut u8 {
    let end: *mut u8;
    unsafe { asm!("ldr {}, =__text_end", out(reg) end) };
    return end;
}

/**
 * Gets the end of main memory from the linker.
 */
pub fn get_memory_end() -> *mut u8 {
    let end: *mut u8;
    unsafe { asm!("ldr {}, =__memory_end", out(reg) end) };
    return end;
}
