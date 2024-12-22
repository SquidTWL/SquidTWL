use crate::{
    gx::{
        dispcnt::{Disp2VramBank, DisplayMode},
        vram::{VRAM_REGISTERS, VramBank, VramControl},
    },
    raw::va::SaneApplyBehaviour,
};

use super::engine::{EngineA, GraphicsEngine};

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

pub struct FramebufferMode<'a> {
    pub engine: &'a mut GraphicsEngine<EngineA>,
    pub vram_bank: FramebufferBank,
}

impl<'a> FramebufferMode<'a> {
    pub(crate) fn new<'b : 'a>(
        engine: &'b mut GraphicsEngine<EngineA>,
        vram_bank: FramebufferBank,
    ) -> FramebufferMode<'a> {
        let idx = vram_bank as usize;
        let reg = VRAM_REGISTERS[idx];
        reg.write(
            VramControl::new()
                .with_enabled(true)
                .with_modeset(0)
                .with_offset(0),  // Offset is ignored in framebuffer mode
        );

        engine.registers.REG_DISPCNT.mutate(|prev| {
            prev.with_display_mode(DisplayMode::Framebuffer)
                .with_framebuffer_vram_block(Disp2VramBank::from_bits(idx as u8))
        });

        return FramebufferMode { engine, vram_bank };
    }
}
