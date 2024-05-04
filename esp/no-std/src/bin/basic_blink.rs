#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal as hal;
use hal::{clock::ClockControl, delay::Delay, gpio::IO, peripherals::Peripherals, prelude::*, riscv::asm::nop};

#[entry]
fn main() -> ! {
    let p = Peripherals::take();
    let system = p.SYSTEM.split();
    let clock = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(p.GPIO, p.IO_MUX);
    let mut pin_4 = io.pins.gpio4.into_push_pull_output();

    let mut is_on = false;
    let delay = Delay::new(&clock);

    loop {
        pin_4.set_state(is_on);

        delay.delay_millis(100);

        is_on = !is_on;
    }
}
