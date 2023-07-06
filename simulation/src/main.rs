use core::time;

use embedded_graphics::{ pixelcolor::BinaryColor, prelude::*, };
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

use mini_marquee::{ draw_frame};

fn main() -> ! {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    let mut window = Window::new("mini_marquee", &output_settings);

    let mut time = 0i32;
    loop{ 
        display.clear(BinaryColor::Off).unwrap();
        draw_frame(&mut display, time).unwrap();
        window.update(&display);
        time += 1;
        std::thread::sleep(time::Duration::from_millis(10));
    }

}
