#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::IO, peripherals::Peripherals, prelude::*, rtc_cntl::Rtc, systimer::SystemTimer};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // setup ultrasonic sensor pins
    let mut trig = io.pins.gpio1.into_push_pull_output();
    let echo = io.pins.gpio0.into_floating_input();

    // setup LED pins
    let mut red_led = io.pins.gpio4.into_push_pull_output();
    let mut yellow_led = io.pins.gpio5.into_push_pull_output();
    let mut green_led = io.pins.gpio6.into_push_pull_output();

    println!("Hello world!");

    loop {
        trig.set_low();
        delay.delay_micros(5u32);

        // trigger the sensor
        trig.set_high();
        delay.delay_micros(10u32);
        trig.set_low();

        // wait for the echo to go high
        while !echo.is_high() {}

        // kick off timer measurement
        let echo_start = SystemTimer::now();

        // wait for the echo to go low
        while !echo.is_low() {}

        // collect current timer count
        let echo_end = SystemTimer::now();

        // calculate the elapsed timer count
        let echo_duration = echo_end.wrapping_sub(echo_start);

        // calculate the distance in cm
        let distance_cm = echo_duration / 16 / 58;

        // turn on the LED based on distance
        if distance_cm < 5 {
            red_led.toggle();
            delay.delay_millis(100u32);
        } else if distance_cm <= 10 {
            red_led.set_high();
            yellow_led.set_low();
            green_led.set_low();
        } else if distance_cm <= 20 {
            yellow_led.set_high();
            red_led.set_low();
            green_led.set_low();
        } else {
            green_led.set_high();
            red_led.set_low();
            yellow_led.set_low();
        }

        println!("Distance: {} cm", distance_cm);
    }
}