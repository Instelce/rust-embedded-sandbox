#![no_std]
#![no_main]

use esp_hal::{clock::ClockControl, delay::Delay, entry, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*};
use ssd1306::{prelude::*, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306};


#[entry]
fn main() -> ! {
    let p = Peripherals::take();
    let system = p.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // configure IO
    let mut io = IO::new(p.GPIO, p.IO_MUX);

    // configure i2c
    let i2c = I2C::new(p.I2C0, io.pins.gpio0, io.pins.gpio1, 100.kHz(), &clocks, None);
 
    // initialize display
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0);

    loop {
    }
}

