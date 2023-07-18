#![no_std]
#![no_main]

use fugit::RateExtU32;
use panic_halt as _;

use waveshare_rp2040_zero::entry;
use waveshare_rp2040_zero::{hal, pac, Pins, XOSC_CRYSTAL_FREQ};
use waveshare_rp2040_zero::hal::pac::interrupt;

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

use ssd1306::{
    prelude::*, rotation::DisplayRotation, size::DisplaySize128x32, I2CDisplayInterface, Ssd1306,
};

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
};

// Globals
/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
/// The USB Serial Device Driver (shared with the interrupt).
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;
static mut DISPLAY_TEXT: [u8; 256] = [0x20; 256];
static mut DISPLAY_TEXT_LEN: usize = 0;

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
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }
    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the USB Communications Class Device driver
    let serial = SerialPort::new(bus_ref);
    unsafe {
        USB_SERIAL = Some(serial);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0xcafe, 0x0001))
        .manufacturer("conjones")
        .product("mini-marquee")
        .serial_number("0x01")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    let mut t = 0i32;
    loop {
        t += 2;
        display.clear(BinaryColor::Off).unwrap();
        mini_marquee::draw_frame(&mut display, t, unsafe {core::str::from_utf8_mut(&mut DISPLAY_TEXT[..DISPLAY_TEXT_LEN]).unwrap()});
        display.flush().unwrap();
    }
}

#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Grab the global objects. This is OK as we only access them under interrupt.
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let serial = USB_SERIAL.as_mut().unwrap();

    // Poll the USB driver with all of our supported USB Classes
    if usb_dev.poll(&mut [serial]) {
        match serial.read(&mut DISPLAY_TEXT) {
            Err(_) | Ok(0) => {}
            Ok(count) => {DISPLAY_TEXT_LEN = count}
        }
    }
}
