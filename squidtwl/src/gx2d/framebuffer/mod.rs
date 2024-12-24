#[cfg(feature = "embedded_graphics")]
pub mod eg;

use crate::{
    gx::{
        dispcnt::{Disp2VramBank, DisplayMode},
        engine::ENGINE_A,
        vram::{VRAM_REGISTERS, VramBank, VramControl},
    },
    raw::va::SaneApplyBehaviour,
};

#[cfg(feature = "embedded_graphics")]
pub use eg::EmbeddedFramebuffer;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FramebufferBank {
    BankA = 0,
    BankB = 1,
    BankC = 2,
    BankD = 3,
}

impl VramBank for FramebufferBank {
    fn bank_base_address(&self) -> usize {
        return match self {
            FramebufferBank::BankA => 0x06800000,
            FramebufferBank::BankB => 0x06820000,
            FramebufferBank::BankC => 0x06840000,
            FramebufferBank::BankD => 0x06860000,
        };
    }
}

/**
 * Enables usage of engine A's framebuffer mode, also known as LCDC mode.
 */
#[derive(Clone, Copy)]
pub struct FramebufferMode {
    pub vram_bank: FramebufferBank,
}

impl FramebufferMode {
    pub fn switch_into(vram_bank: FramebufferBank) -> FramebufferMode {
        let idx = vram_bank as usize;
        let reg = VRAM_REGISTERS[idx];
        reg.write(
            VramControl::new()
                .with_enabled(true)
                .with_modeset(0)
                .with_offset(0), // Offset is ignored in framebuffer mode
        );

        ENGINE_A.REG_DISPCNT.mutate(|prev| {
            prev.with_display_mode(DisplayMode::Framebuffer)
                .with_framebuffer_vram_block(Disp2VramBank::from_bits(idx as u8))
        });

        return FramebufferMode { vram_bank };
    }
}
