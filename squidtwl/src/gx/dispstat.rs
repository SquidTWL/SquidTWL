use bitfield_struct::bitfield;
use voladdress::{Safe, VolAddress};

/** The bits for the Display Status register */
#[bitfield(u16)]
pub struct DisplayStatus {
    /** If true, currently in vertical blank */
    #[bits(access = RO)]
    pub is_vblank: bool, // 0

    /** If true, currently in horizontal blank */
    #[bits(access = RO)]
    pub is_hblank: bool, // 1

    /** If true, the vertical counter == ``vcount_match_value`` */
    #[bits(access = RO)]
    pub is_vcount_matching: bool, // 2

    /** Enables/disables vertical blank IRQs */
    pub enable_vblank_irq: bool, // 3
    /** Enables/disables horizontal blank IRQs */
    pub enable_hblank_irq: bool, // 4
    /** Enables/disables IRQs when the vertical counter == ``vcount_match_value`` */
    pub enable_vcount_irq: bool, // 5

    /** TWL only: if the LCD hardware is ready. Unused on NTR */
    pub lcd_init_ready: bool, // 6

    /** The value for the vertical count match IRQ */
    #[bits(9)]
    pub vcount_match_value: u16,
}

/** Raw access to the Display Status register. */
pub const REG_DISPSTAT: VolAddress<DisplayStatus, Safe, Safe> =
    unsafe { VolAddress::new(0x4000004) };
