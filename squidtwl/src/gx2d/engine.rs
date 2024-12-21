// The logical way to do this would be with inheritance.
// GraphicsEngine is inherited by EngineA (which adds LCDC) and EngineB (which adds nothing).
// But 20 years of circlejerking by so-called programmers who are shadowboxing Java 6 means that
// we have to do this the stupid way instead.

use core::marker::PhantomData;

use sealed::sealed;
use voladdress::Safe;

use crate::gx::dispcnt::DisplayControl;

use super::framebuffer::{FramebufferBank, FramebufferMode};

/** distinct enums when? */
#[sealed]
pub trait Engine2D {}

#[allow(non_snake_case)]
pub(crate) struct EngineRegisters {
    pub REG_DISPCNT: voladdress::VolAddress<DisplayControl, Safe, Safe>,
}

pub struct GraphicsEngine<T: Engine2D> {
    pub(crate) registers: EngineRegisters,
    _unused: PhantomData<T>,
}

// fake structs used to separate impl for Engine A and B
pub struct EngineA {}
#[sealed]
impl Engine2D for EngineA {}

pub struct EngineB {}
#[sealed]
impl Engine2D for EngineB {}

// == Actual code impls below here ==

impl EngineRegisters {
    fn new(base: usize) -> EngineRegisters {
        return unsafe {
            let dispcnt = voladdress::VolAddress::new(base);

            EngineRegisters { REG_DISPCNT: dispcnt }
        };
    }
}

// common functionality between both
impl<T: Engine2D> GraphicsEngine<T> {}

// engine A exclusive functionality
impl GraphicsEngine<EngineA> {
    pub(crate) fn new() -> Self {
        return GraphicsEngine {
            registers: EngineRegisters::new(0x4000000),
            _unused: PhantomData,
        };
    }

    /**
     * Switches this engine to framebuffer mode using VRAM bank A.
     */
    pub fn as_framebuffer(&mut self) -> FramebufferMode<'_> {
        return FramebufferMode::new(self, FramebufferBank::BankA);
    }

    /**
     * Switches this engine to framebuffer mode using the provided VRAM bank.
     * 
     * This is useful if bank A/B/C will be used for something else.
     */
    pub fn as_framebuffer_with_bank(&mut self, bank: FramebufferBank) -> FramebufferMode<'_> {
        return FramebufferMode::new(self, bank);
    }
}

// engine B exclusive functionality
impl GraphicsEngine<EngineB> {
    pub(crate) fn new() -> Self {
        return GraphicsEngine {
            registers: EngineRegisters::new(0x4001000),
            _unused: PhantomData,
        };
    }
}
