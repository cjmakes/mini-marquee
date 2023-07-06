use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

// TODO(conjones) come up with a closed form solution to render the scene bacause that would be
// more reusable and also pretty cool
pub struct Scene {
    pub x1: i32,
    pub x2: i32,
}

pub fn draw_frame<D>(display: &mut D, s: &mut Scene) -> core::result::Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>
{
    // TODO(conjones) what trait requirements do I have to add to clear the frame here?
    s.x1 -= 1;
    if s.x1 == -270 {
        s.x1 = 270
    }
    s.x2 -= 1;
    if s.x2 == -270 {
        s.x2 = 270
    }

    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    Text::new("hi caroline, i love you <3", Point::new(s.x1, 20), style).draw(display)?;
    Text::new("hi caroline, i love you <3", Point::new(s.x2, 20), style).draw(display)?;

    Ok(())
}
