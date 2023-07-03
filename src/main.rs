#![no_std]
#![no_main]

use core::iter::once;

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

use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};

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
    let mut ws = Ws2812::new(
        // The onboard NeoPixel is attached to GPIO pin #16 on the Feather RP2040.
        pins.neopixel.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

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

    let mut pos = 0;
    loop {
        if pos == 4096 {
            pos = 0;
            display.clear_buffer();
            ws.write(brightness(once(RGB8 { r: 255, g: 0, b: 0 }), 10))
                .unwrap();
            delay.start(500.millis());
            let _ = nb::block!(delay.wait());

            ws.write(brightness(once(RGB8 { r: 0, g: 255, b: 0 }), 10))
                .unwrap();
            delay.start(500.millis());
            let _ = nb::block!(delay.wait());

            ws.write(brightness(once(RGB8 { r: 0, g: 0, b: 255 }), 10))
                .unwrap();
            delay.start(500.millis());
            let _ = nb::block!(delay.wait());
        }

        Pixel(Point::new(pos / 32, pos % 32), BinaryColor::On)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        pos += 1;
        delay.start(10.millis());
        let _ = nb::block!(delay.wait());

        if pos % 32 == 0 {
            pos += 32;

            ws.write(brightness(once(RGB8 { r: 255, g: 0, b: 0 }), 10))
                .unwrap();
            delay.start(100.millis());
            let _ = nb::block!(delay.wait());

            ws.write(brightness(once(RGB8 { r: 0, g: 0, b: 255 }), 10))
                .unwrap();
            delay.start(100.millis());
            let _ = nb::block!(delay.wait());
        }
    }
}
