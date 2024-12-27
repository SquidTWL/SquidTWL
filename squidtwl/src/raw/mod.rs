pub mod arm;
pub mod supervisor;
pub mod va;

use bitfield_struct::bitfield;
// re-exports
pub use supervisor::SWI_Halt;

pub use arm::{CurrentProgramState, ProcessorMode};
use voladdress::{Safe, VolAddress};

/** Enumeration of the possible WRAM bank assignments. */
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum WramBank {
    AllArm9 = 0,
    SecondHalfArm9 = 1,
    FirstHalfArm9 = 2,
    AllArm7 = 3,
}

impl WramBank {
    pub const fn into_bits(self) -> u8 {
        return self as u8;
    }

    pub const fn from_bits(value: u8) -> WramBank {
        return match value {
            0 => Self::AllArm9,
            1 => Self::FirstHalfArm9,
            2 => Self::SecondHalfArm9,
            3 => Self::AllArm7,
            _ => unreachable!(),
        };
    }
}

#[bitfield(u8)]
pub struct WramControl {
    #[bits(2)]
    pub bank: WramBank,

    #[bits(6)]
    _pad: u8,
}

/**
 * Controls the assignment of the working RAM between the ARM9 and the ARM7.
 */
pub const WRAMCTL: VolAddress<WramControl, Safe, Safe> = unsafe { VolAddress::new(0x04000247) };
