use core::fmt::Display;

use voladdress::Safe;

use crate::gx::dispctl::DisplayControl;

/**
 * Allows easy access to the engine-specific graphics registers.
 */
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct EngineRegisters {
    pub REG_DISPCTL: voladdress::VolAddress<DisplayControl, Safe, Safe>,
}

impl Display for EngineRegisters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return write!(f, "base: {}", self.REG_DISPCTL.as_ptr() as usize);
    }
}

// == Actual code impls below here ==

impl EngineRegisters {
    const fn new(base: usize) -> EngineRegisters {
        return unsafe {
            let dispctl = voladdress::VolAddress::new(base);

            EngineRegisters {
                REG_DISPCTL: dispctl,
            }
        };
    }
}

const ENGINE_A: EngineRegisters = EngineRegisters::new(0x4000000);
const ENGINE_B: EngineRegisters = EngineRegisters::new(0x4001000);

#[derive(Clone, Copy, PartialEq)]
pub enum GraphicsEngine {
    EngineA,
    EngineB,
}

pub trait HasRegisters {
    fn regs(&self) -> &'static EngineRegisters;
}

impl HasRegisters for GraphicsEngine {
    fn regs(&self) -> &'static EngineRegisters {
        return match self {
            GraphicsEngine::EngineA => &ENGINE_A,
            GraphicsEngine::EngineB => &ENGINE_B,
        };
    }
}
