#![no_std]
#![no_main]

use bsp::entry;
use rp_pico as bsp;
use fugit::RateExtU32;
use embedded_hal::digital::v2::OutputPin;

use core::fmt::Write;
use core::panic::PanicInfo;

use bsp::hal::i2c::I2C;

use bsp::hal::{
    clocks::init_clocks_and_plls,
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

//use embedded_graphics::{
//    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
//    pixelcolor::BinaryColor,
//    prelude::*,
//    text::{Baseline, Text},
//};

use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};


#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin = pins.gpio14.into_function::<bsp::hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio15.into_function::<bsp::hal::gpio::FunctionI2C>();

    let i2c = I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        100.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    ).into_terminal_mode(); //into_buffered_graphics_mode();

    display.init().unwrap();

    let _ = display.clear();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, 125_000_000);
    let mut led_pin = pins.led.into_push_pull_output();

    let mut x = 0;

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);

//        let _ = display.write_str("blarb"); 
        let _ = display.write_fmt(format_args!("{:?}", x));

        led_pin.set_low().unwrap();
        delay.delay_ms(500);
        x = x + 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
