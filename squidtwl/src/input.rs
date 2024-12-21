use core::ops::Not;

use bitfield_struct::bitfield;
use voladdress::{Safe, Unsafe, VolAddress};

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

/** Similar to ``RawKeyInput``, but for controlling key interrupts */
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

/** Warning: 0 = pressed, 1 = released */
pub const REG_KEYINPUT: VolAddress<u16, Unsafe, ()> = unsafe { VolAddress::new(0x4000130) };
pub const REG_KEYCTL: VolAddress<RawKeyControl, Safe, Safe> = unsafe { VolAddress::new(0x4000312) };

// TODO: merge raw input and arm7 input.
pub fn read_key_input() -> RawKeyInput {
    let value = unsafe { REG_KEYINPUT.read() };
    let bits = value.not() & 0b1111111111;
    return RawKeyInput::from_bits(bits);
}
