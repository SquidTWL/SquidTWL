use core::ops::Not;

use bitflags::bitflags;
use thiserror::Error;
use voladdress::{Safe, VolAddress};

use crate::raw::SWI_Halt;

#[cfg(feature = "irq")]
pub mod irq;

#[cfg(not(feature = "irq"))]
mod irq_weak {
    use super::REG_IF;

    /**
     * The default IRQ handler that does nothing. Weak symbol, so it can be overwritten with a
     * user-defined symbol.
     */
    #[linkage = "weak"]
    pub fn irq_handler() {
        REG_IF.write(REG_IF.read());
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq)]
    #[repr(transparent)]
    pub struct ImeFlags : u32 {
        const ENABLED = 0b1;
        const DISABLED = 0b0;
    }
}

pub static REG_IME: VolAddress<ImeFlags, Safe, Safe> = unsafe { VolAddress::new(0x4000208) };

/**
 * Disables the Interrupt Master Enable, turning all interrupts off.
 *
 * This ignores the value of ``IE``.
 */
pub fn disable_master_flag() -> bool {
    let prev = REG_IME.read();
    REG_IME.write(ImeFlags::DISABLED);
    return prev != ImeFlags::DISABLED;
}

/**
 * Enables the Interrupt Master Enable, allowing any enabled interrupts to be fired.
 */
pub fn enable_master_flag() -> bool {
    let prev = REG_IME.read();
    REG_IME.write(ImeFlags::ENABLED);
    return prev != ImeFlags::DISABLED;
}

bitflags! {
    /**
     * Bit flags for the possible Interrupt Enable/Interrupt Request Flags bits.
     */
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct InterruptBits : u32 {
        const LCD_VERTICAL_BLANK = 1 << 0;
        const LCD_HORIZONTAL_BLANK = 1 << 1;
        /** See the docs for the DISPCNT register for more information. */
        const LCD_VCOUNTER_MATCH = 1 << 2;

        const TIMER_OVERFLOW_0 = 1 << 3;
        const TIMER_OVERFLOW_1 = 1 << 4;
        const TIMER_OVERFLOW_2 = 1 << 5;
        const TIMER_OVERFLOW_3 = 1 << 6;
        // (7 << 1): NDS7 only: SIO/RCNT/RTC (Real Time Clock)
        const DMA_0 = 1 << 8;
        const DMA_1 = 1 << 9;
        const DMA_2 = 1 << 10;
        const DMA_3 = 1 << 11;
        const KEYPAD = 1 << 12;
        /** Nitro-only */
        const NTR_GBA_SLOT = 1 << 13;
        // 14/15: TWL-specific, not used anyway
        const IPC_SYNC = 1 << 16;
        const IPC_SEND_EMPTY = 1 << 17;
        const IPC_RECV_EMPTY = 1 << 18;
        const CARTRIDGE_XFER_COMPLETE = 1 << 19;
        // TODO: rest of the TWL bits
    }
}

pub static REG_IE: VolAddress<InterruptBits, Safe, Safe> = unsafe { VolAddress::new(0x4000210) };
pub static REG_IF: VolAddress<InterruptBits, Safe, Safe> = unsafe { VolAddress::new(0x4000214) };

/**
 * Enables the interrupts specified by the provided mask.
 *
 * This will ``AND`` the previous bits in the IE register with the ones provided.
 */
pub fn enable_interrupts(mask: InterruptBits) {
    let mut regval = REG_IE.read();
    regval |= mask;
    REG_IE.write(regval);
}

/**
 * Disables the interrupts specified by the provided mask.
 *
 * This will unset any bits in the IE register that are set in the parameter.
 */
pub fn disable_interrupts(mask: InterruptBits) {
    let mut regval = REG_IE.read();
    regval &= mask.not();
    REG_IE.write(regval);
}

#[derive(Debug, Error)]
#[error("No bits specified in mask, this will lock up the processor!")]
pub struct NoBitsError;

/**
 * Waits for a set of interrupts specified by the provided mask.
 *
 * This operation is inherently unsafe, as some interrupt sources require setting flags in their
 * own registers to avoid locking the processor up.
 */
pub unsafe fn wait_for_interrupts(mask: InterruptBits) -> Result<(), NoBitsError> {
    if mask.is_empty() {
        let prev = REG_IE.read();
        if prev.is_empty() {
            return Err(NoBitsError);
        }
    }

    enable_interrupts(mask);
    enable_master_flag();

    unsafe {
        SWI_Halt();
    }

    disable_master_flag();
    disable_interrupts(mask);

    return Ok(());
}
