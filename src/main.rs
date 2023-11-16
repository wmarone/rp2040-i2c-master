#![no_std]
#![no_main]

use rp_pico;
use rp_pico::{
    hal,
    entry,
    Pins,
};

use hal::{
    gpio,
    i2c::I2C,
    clocks,
    pac,
    sio::Sio,
    watchdog::Watchdog,
    usb::UsbBus,
};

use pac::{
    Peripherals,
    CorePeripherals
};

use usbd_serial::SerialPort;

use ssd1306::{
    prelude::*,
    I2CDisplayInterface,
    Ssd1306
};

use usb_device::bus::UsbBusAllocator;
use usb_device::device::UsbDeviceBuilder;
use usb_device::device::UsbVidPid;

use fugit::RateExtU32;
use embedded_hal::digital::v2::OutputPin;

use core::fmt::Write;
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    let sio = Sio::new(peripherals.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    
    let clocks = clocks::init_clocks_and_plls(
        external_xtal_freq_hz,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let usb = UsbBus::new(
        peripherals.USBCTRL_REGS,
        peripherals.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut peripherals.RESETS,
    );

    let usb_bus = UsbBusAllocator::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x0330, 0x1D1D))
                                            .manufacturer("side-7")
                                            .product("Serial Port")
                                            .serial_number("1234567")
                                            .device_class(2)
                                            .build();

    let pins = Pins::new(
         peripherals.IO_BANK0,
         peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let sda_pin = pins.gpio14.into_function::<gpio::FunctionI2C>();
    let scl_pin = pins.gpio15.into_function::<gpio::FunctionI2C>();

    let i2c = I2C::i2c1(
        peripherals.I2C1,
        sda_pin,
        scl_pin,
        100.kHz(),
        &mut peripherals.RESETS,
        125_000_000.Hz(),
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    ).into_terminal_mode();

    display.init().unwrap();

    let _ = display.clear();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, 125_000_000);
    let mut led_pin = pins.led.into_push_pull_output();

    let mut x = 0;

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);

        let _ = display.write_fmt(format_args!("{:?} ", x));

        led_pin.set_low().unwrap();
        delay.delay_ms(500);
        x = x + 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
