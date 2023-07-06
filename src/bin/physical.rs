#![no_std]
#![no_main]

use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_hal::timer::CountDown;
use fugit::ExtU32;
use fugit::RateExtU32;
use panic_halt as _;
use waveshare_rp2040_zero::entry;
use waveshare_rp2040_zero::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        i2c, pac,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = timer.count_down();

    // Configure the addressable LED
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let i2c = i2c::I2C::i2c1(
        pac.I2C1,
        pins.gp14.into_mode(), // sda
        pins.gp15.into_mode(), // scl
        400.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    let interface = I2CDisplayInterface::new(i2c);

    let display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0);
    let mut display = display.into_buffered_graphics_mode();
    display.init().unwrap();
    let mut delay = timer.count_down();

    let mut pos = 0;
    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    loop {
        if pos == -260 {
            pos = 0;
        }
        display.clear_buffer();
        Text::new(
            "hi caroline, i love you <3",
            Point::new(pos, 20),
            style,
        )
        .draw(&mut display)
        .unwrap();
        pos -= 1;
        display.flush().unwrap();

        delay.start(5.millis());
        let _ = nb::block!(delay.wait());
    }
}
