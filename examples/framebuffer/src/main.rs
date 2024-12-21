#![no_std]
#![no_main]

use embedded_graphics::Drawable;
use embedded_graphics::{
    pixelcolor::Bgr555,
    prelude::{Point, Primitive},
    primitives::{PrimitiveStyle, Triangle},
};
use squidtwl::gx::wait_for_vertical_blank;
use squidtwl::gx2d::{Graphics2D, eg::EmbeddedFramebuffer};

#[unsafe(no_mangle)]
extern "C" fn main() {
    let mut graphics = Graphics2D::new().unwrap();
    let mut fb = graphics.engine_a.as_framebuffer();
    let mut lcd = EmbeddedFramebuffer::wrap(&mut fb);

    let tri = Triangle::new(Point::new(10, 10), Point::new(100, 10), Point::new(10, 100))
        .into_styled(PrimitiveStyle::with_fill(Bgr555::new(15, 0, 15)));
    tri.draw(&mut lcd).unwrap();

    loop {
        wait_for_vertical_blank();
    }
}
