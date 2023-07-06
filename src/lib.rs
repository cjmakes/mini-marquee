#![no_std]
use core::convert::TryInto;
use core::result::Result::Ok;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

pub fn draw_frame<D>(display: &mut D, t: i32) -> core::result::Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>
{
    // TODO(conjones) what trait requirements do I have to add to clear the frame here?

    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let txt = Text::new("hi caroline, i love you <3", Point::new(0, 20), style);

    let dw:i32 = display.bounding_box().size.width.try_into().unwrap();
    let tw: i32 = txt.bounding_box().size.width.try_into().unwrap();

    let x1:i32 = dw - (t % (dw + tw));

    txt.translate(Point::new(x1, 0)).draw(display)?;
    Ok(())
}
