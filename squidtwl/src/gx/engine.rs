use voladdress::Safe;

use crate::gx::dispcnt::DisplayControl;

/**
 * Allows easy access to the engine-specific graphics registers.
 */
#[allow(non_snake_case)]
pub struct EngineRegisters {
    pub REG_DISPCNT: voladdress::VolAddress<DisplayControl, Safe, Safe>,
}

// == Actual code impls below here ==

impl EngineRegisters {
    const fn new(base: usize) -> EngineRegisters {
        return unsafe {
            let dispcnt = voladdress::VolAddress::new(base);

            EngineRegisters {
                REG_DISPCNT: dispcnt,
            }
        };
    }
}

pub const ENGINE_A: EngineRegisters = EngineRegisters::new(0x4000000);
pub const ENGINE_B: EngineRegisters = EngineRegisters::new(0x4001000);
