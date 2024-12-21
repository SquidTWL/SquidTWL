use bitfield_struct::bitfield;
use voladdress::{Safe, VolAddress};

/** Common trait for the various VRAM bank enums. */
pub trait VramBank {
    /** The base address of this bank, e.g. 0x06800000. */
    fn bank_base_address(&self) -> usize;
}

#[bitfield(u8)]
pub struct VramControl {
    #[bits(3)]
    pub mst: u8,

    #[bits(2)]
    pub offset: u8,

    #[bits(2)]
    _pad: u8,

    #[bits(default = true)]
    pub enabled: bool,
}

const VREG_BASE_ADDR: usize = 0x4000240;

pub const VRAM_CTL_A: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR) };
pub const VRAM_CTL_B: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR + 1) };
pub const VRAM_CTL_C: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR + 2) };
pub const VRAM_CTL_D: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR + 3) };
pub const VRAM_CTL_E: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR + 4) };
pub const VRAM_CTL_F: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR + 5) };
pub const VRAM_CTL_G: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR + 6) };
// not a mistake; gbatek gives these as 248h and 249h?
pub const VRAM_CTL_H: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR + 8) };
pub const VRAM_CTL_I: VolAddress<VramControl, Safe, Safe> =
    unsafe { VolAddress::new(VREG_BASE_ADDR + 9) };

pub const VRAM_REGISTERS: [VolAddress<VramControl, Safe, Safe>; 9] = [
    VRAM_CTL_A, VRAM_CTL_B, VRAM_CTL_C, VRAM_CTL_D, VRAM_CTL_E, VRAM_CTL_F, VRAM_CTL_G, VRAM_CTL_H,
    VRAM_CTL_I,
];
