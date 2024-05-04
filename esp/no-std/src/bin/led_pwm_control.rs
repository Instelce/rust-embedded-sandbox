#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal as hal;
use esp_println::{print, println};
use hal::{clock::ClockControl, delay::Delay, gpio::{GpioPin, Output, PushPull, IO}, ledc::{channel, timer, LSGlobalClkSource, LowSpeed, LEDC}, peripherals::Peripherals, prelude::*, riscv::asm::nop};

#[entry]
fn main() -> ! {
    let p = Peripherals::take();
    let system = p.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(p.GPIO, p.IO_MUX);
    let mut ledc = LEDC::new(p.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let pin_4 = io.pins.gpio4.into_push_pull_output();

    let mut lstimer = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer.configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50.Hz()
    }).unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel0, pin_4);
    channel0.configure(channel::config::Config {
        timer: &lstimer,
        duty_pct: 10,
        pin_config: channel::config::PinConfig::PushPull
    }).unwrap();

    let mut intensity: i32 = 0;
    let mut dir = 0;
    let delay = Delay::new(&clocks);

    loop {
        println!("{}", intensity);
        channel0.set_duty(intensity as u8).ok();
        delay.delay_millis(50);
        if dir == 0 {
            intensity += 5;
        } else if dir == 1 {
            intensity -= 5;
        }
        if intensity >= 95 {
            dir = 1;
        } else if intensity <= 0 {
            dir = 0;
        }
    }
}
