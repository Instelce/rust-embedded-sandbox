#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal as hal;
use esp_println::{print, println};
use hal::{analog::adc::{AdcCalCurve, AdcCalLine, AdcConfig, Attenuation, ADC}, clock::ClockControl, delay::Delay, gpio::IO, ledc::{channel, timer, LSGlobalClkSource, LowSpeed, LEDC}, peripherals::{Peripherals, ADC1}, prelude::*};

#[entry]
fn main() -> ! {
    let p = Peripherals::take();
    let system = p.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let io = IO::new(p.GPIO, p.IO_MUX);

    // create ADC instance to read potentiometer value
    let mut adc_config = AdcConfig::<ADC1>::new();
    let analog_pin0 = io.pins.gpio0.into_analog();
    let mut potentiometer = adc_config.enable_pin_with_cal::<_, AdcCalLine<ADC1>>(analog_pin0, Attenuation::Attenuation0dB);
    let mut adc = ADC::new(p.ADC1, adc_config);

    // setup LED PWM Controller for pin 4
    let mut ledc = LEDC::new(p.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let led = io.pins.gpio4.into_push_pull_output();

    let mut lstimer = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer.configure(timer::config::Config {
        duty: timer::config::Duty::Duty14Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 50.Hz()
    }).unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel0, led);
    channel0.configure(channel::config::Config {
        timer: &lstimer,
        duty_pct: 10,
        pin_config: channel::config::PinConfig::PushPull
    }).unwrap();

    let mut percent: u8 = 0;

    loop {
        let potentiometer_value: u16 = nb::block!(adc.read_oneshot(&mut potentiometer)).unwrap();
        percent = (((potentiometer_value as u32) * 100) / 834) as u8;

        println!("Potentiometer value : {} | Percentage : {}%", potentiometer_value, percent);

        if percent >= 100 {
            percent = 99;
        }

        channel0.set_duty(percent).ok();
    }
}
