#![no_std]
#![no_main]

use fugit::RateExtU32;
use panic_halt as _;

use waveshare_rp2040_zero::entry;
use waveshare_rp2040_zero::{hal, pac, Pins, XOSC_CRYSTAL_FREQ};

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

use ssd1306::{
    prelude::*, rotation::DisplayRotation, size::DisplaySize128x32, I2CDisplayInterface, Ssd1306,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
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

    let sio = hal::Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let i2c = hal::i2c::I2C::i2c1(
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

    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    display.clear_buffer();
    display.flush().unwrap();

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0xcafe, 0x27dd))
        .manufacturer("conjones")
        .product("mini-marquee")
        .serial_number("0x01")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    let _timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    loop {
        if usb_dev.poll(&mut [&mut serial]) {
            let mut buf = [0u8; 64];
            match serial.read(&mut buf) {
                Err(_) | Ok(0) => {}
                Ok(count) => {
                    display.clear(BinaryColor::Off).unwrap();
                    let buf = core::str::from_utf8_mut(&mut buf[..count]).unwrap();
                    Text::new(buf, Point::new(0, 20), style)
                        .draw(&mut display)
                        .unwrap();
                    display.flush().unwrap();
                }
            }
        }
    }
}
