use dispstat::REG_DISPSTAT;

use crate::{interrupt::{wait_for_interrupts, InterruptBits}, raw::va::SaneApplyBehaviour};

pub mod dispstat;
pub mod dispcnt;
pub mod vram;

/**
 * Waits for the next vertical blank interrupt.
 */
pub fn wait_for_vertical_blank() {
    REG_DISPSTAT.mutate(|prev| prev.with_enable_vblank_irq(true));
    unsafe {
        // no error possible here!
        wait_for_interrupts(InterruptBits::LCD_VERTICAL_BLANK).unwrap();
    }
    REG_DISPSTAT.mutate(|prev| prev.with_enable_vblank_irq(false));
}
