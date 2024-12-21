//! Infrastructure for handling individual interrupts.

use super::{InterruptBits, REG_IF};

static IRQ_TABLE: spin::once::Once<[fn() -> bool; 32]> = spin::once::Once::new();

/**
 * Sets the global IRQ table. This will do nothing on any subsequent attempt to call it.
 */
pub fn set_irq_table(table: [fn() -> bool; 32]) {
    IRQ_TABLE.call_once(|| table);
}

/**
 * The root IRQ handler function. The name of this function is hardcoded into the startup assembly
 * and placed at the appropriate area of the DTCM.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn irq_handler() {
    let raw_if = REG_IF.read().bits();

    if let Some(table) = IRQ_TABLE.get() {
        // seems to be identical to how the official SDK does it, so it's probably correct
        // that there's only one bit set in REG_IF at once.
        let irq_number = raw_if.trailing_zeros() as usize;
        let handler = table[irq_number];

        if handler() {
            REG_IF.write(InterruptBits::from_bits_truncate(1 << irq_number));
        }

        return;
    }

    // no IRQ table setup (fuck you), just clear IF and be done with it
    REG_IF.write(InterruptBits::from_bits_retain(raw_if));
}
