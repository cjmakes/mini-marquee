use core::time;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use mini_marquee::draw_frame;

fn main() -> ! {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    let mut window = Window::new("mini_marquee", &output_settings);

    let mut time = 0i32;
    loop {
        draw_frame(&mut display, time, "long scrolling text").unwrap();
        window.update(&display);
        time += 1;
        std::thread::sleep(time::Duration::from_millis(10));
    }
}
