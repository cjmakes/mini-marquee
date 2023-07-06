use core::time;

use embedded_graphics::{ pixelcolor::BinaryColor, prelude::*, };
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

use mini_marquee::{Scene, draw_frame};

fn main() -> ! {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let mut scene = Scene{x1: 0, x2: 280};
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    let mut window = Window::new("mini_marquee", &output_settings);

    loop{ 
        display.clear(BinaryColor::Off).unwrap();
        draw_frame(&mut display, &mut scene).unwrap();
        window.update(&display);
        std::thread::sleep(time::Duration::from_millis(10));
    }

}
