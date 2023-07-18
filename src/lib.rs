#![no_std]
use core::convert::TryInto;
use core::result::Result::Ok;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

pub fn draw_frame<D>(display: &mut D, t: i32, txt: &str) -> core::result::Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>
{
    display.clear(BinaryColor::Off)?;

    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let txt = Text::new(txt, Point::new(0, 20), style);

    let dw:i32 = display.bounding_box().size.width.try_into().unwrap();
    let tw: i32 = txt.bounding_box().size.width.try_into().unwrap();

    let x1:i32 = dw - (t % (dw + tw));

    txt.translate(Point::new(x1, 0)).draw(display)?;
    Ok(())
}
