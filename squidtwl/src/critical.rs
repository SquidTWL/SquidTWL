use critical_section::set_impl;

use crate::{
    interrupt::{ImeFlags, REG_IME},
    raw::{CurrentProgramState, ProcessorMode},
};

struct TwlCriticalSection;

unsafe impl critical_section::Impl for TwlCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        let cpsr = CurrentProgramState::get();
        if cpsr.mode() == ProcessorMode::IRQ {
            // already in interrupt mode, nothing to do here
            return 0;
        }

        let prev = REG_IME.read();
        REG_IME.write(ImeFlags::ENABLED);
        return prev.bits();
    }

    unsafe fn release(restore_state: critical_section::RawRestoreState) {
        let cpsr = CurrentProgramState::get();
        if cpsr.mode() == ProcessorMode::IRQ {
            return;
        }

        let prev = ImeFlags::from_bits_truncate(restore_state);
        REG_IME.write(prev);
    }
}

set_impl!(TwlCriticalSection);
