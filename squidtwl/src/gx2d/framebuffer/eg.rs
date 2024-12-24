/*!
 * Contains code to use the framebuffer mode with the ``embedded_graphics`` library.
 */

use embedded_graphics::{
    Pixel,
    pixelcolor::Bgr555,
    prelude::{DrawTarget, IntoStorage, OriginDimensions, Size},
};
use voladdress::{Safe, VolBlock};

use crate::{gx::vram::VramBank, gx2d::framebuffer::FramebufferMode};

pub struct EmbeddedFramebuffer {
    vram: VolBlock<u16, Safe, Safe, 131072>,
}

impl EmbeddedFramebuffer {
    pub fn wrap(mode: &FramebufferMode) -> EmbeddedFramebuffer {
        let block = unsafe { VolBlock::new(mode.vram_bank.bank_base_address()) };
        return EmbeddedFramebuffer { vram: block };
    }
}

impl OriginDimensions for EmbeddedFramebuffer {
    fn size(&self) -> embedded_graphics::prelude::Size {
        return Size::new(256, 192);
    }
}

impl DrawTarget for EmbeddedFramebuffer {
    type Color = Bgr555;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let bound = self.size();

        for Pixel(coord, colour) in pixels {
            if coord.x < 0 || coord.x > bound.width as i32 {
                continue;
            }

            if coord.y < 0 || coord.y > bound.height as i32 {
                continue;
            }

            let pos = coord.x + (coord.y * 256);
            let offset = self.vram.index(pos as usize);
            offset.write(colour.into_storage());
        }

        return Ok(());
    }
}
