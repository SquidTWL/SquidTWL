use bitfield_struct::bitfield;
use voladdress::{Safe, VolAddress};

/**
 * The bits for the raw, ARM9-side input controller.
 */
#[bitfield(u16)]
#[derive(PartialEq, Eq)]
pub struct RawKeyInput {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub shoulder_l: bool,
    pub shoulder_r: bool,

    #[bits(6)]
    _pad: u8,
}

/** Similar to ``RawKeyInput``, but for controlling key interrupts too. */
#[bitfield(u16)]
#[derive(PartialEq, Eq)]
pub struct RawKeyControl {
    #[bits(10)]
    pub keys: RawKeyInput,

    #[bits(4)]
    _pad: u8,

    pub irq_enable: bool,
    pub logical_and: bool,
}

pub const REG_KEYINPUT: VolAddress<RawKeyInput, Safe, ()> = unsafe { VolAddress::new(0x4000130) };
pub const REG_KEYCTL: VolAddress<RawKeyControl, Safe, Safe> = unsafe { VolAddress::new(0x4000312) };
