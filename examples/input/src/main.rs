/*!
 * An example showing how to do input, using the framebuffer mode.
 * 
 * This is a bit glitchy due to the low speed of the framebuffer mode.
 */

#![no_std]
#![no_main]

use embedded_graphics::{image::Image, pixelcolor::Bgr555, prelude::*, primitives::{PrimitiveStyle, StyledDrawable}};
use squidtwl::{
    gx::wait_for_vertical_blank,
    gx2d::{eg::EmbeddedFramebuffer, Graphics2D},
    input::read_key_input,
};
use tinybmp::Bmp;

const BITMAP: &[u8] = include_bytes!("icon.bmp");

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let mut graphics = Graphics2D::new().unwrap();
    let mut fb = graphics.engine_a.as_framebuffer();
    let mut eg = EmbeddedFramebuffer::wrap(&mut fb);

    let bmp: Bmp<'_, Bgr555> = Bmp::from_slice(BITMAP).unwrap();
    let mut image = Image::new(&bmp, Point::zero());
    let clear_style = PrimitiveStyle::with_fill(Bgr555::BLACK);

    let mut dirty = false;

    loop {
        wait_for_vertical_blank();

        let input = read_key_input();

        if dirty && input.into_bits() == 0 {
            eg.bounding_box().draw_styled(&clear_style, &mut eg).unwrap();
            dirty = false;
        }

        if input.up() {
            image = image.translate(Point::new(0, -10));
            dirty = true;
        }
        else if input.down() {
            image = image.translate(Point::new(0, 10));
            dirty = true;
        }
        else if input.left() {
            image = image.translate(Point::new(-10, 0));
            dirty = true;
        }
        else if input.right() {
            image = image.translate(Point::new(10, 0));
            dirty = true;
        }

        image.draw(&mut eg).unwrap();
    }
}
