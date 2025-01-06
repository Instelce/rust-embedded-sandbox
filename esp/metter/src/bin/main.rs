#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt;

use critical_section::Mutex;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::iso_8859_1::FONT_10X20;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::text::{Alignment, Baseline, Text};
use esp_backtrace as _;
use esp_hal::delay::{self, Delay};
use esp_hal::gpio::{Event, Input, Io, Level, Output, Pull};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use esp_hal::spi::slave::Spi;
use esp_hal::spi::SpiMode;
use esp_hal::timer::systimer::SystemTimer;
use esp_println::println;
use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};

type StaticPin<T> = Mutex<RefCell<Option<T>>>;

static BUTTON: StaticPin<Input> = Mutex::new(RefCell::new(None));
static TRIGGER: StaticPin<Output> = Mutex::new(RefCell::new(None));
// static DISPLAY_CENTER: Point = ;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(interrupt_handler);

    // Led and button
    let mut led = Output::new(peripherals.GPIO7, Level::Low);
    let mut button = Input::new(peripherals.GPIO6, Pull::Down);
    button.listen(Event::FallingEdge);
    static_replace(&BUTTON, button);

    // Ultrasonic sensor
    let trig = Output::new(peripherals.GPIO1, Level::Low);
    static_replace(&TRIGGER, trig);
    let echo = Input::new(peripherals.GPIO0, Pull::None);

    // Setup and initialize display
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .with_scl(peripherals.GPIO4)
        .with_sda(peripherals.GPIO5);
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().expect("Cannot innitialize the display");

    // Text styles
    let heading_text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();
    let base_text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut distance_buffer = [0u8; 64];
    let mut distance_text = "";
    let mut distance_text_style = heading_text_style;

    // Title text
    center_text("Metter", heading_text_style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        // Wait trigger
        while !echo.is_high() {}

        // Nice, the wave is sended
        let echo_start = SystemTimer::now();
        led.set_high();

        // Wait the wave to get back
        while !echo.is_low() {}

        // Wave received
        // Lets proccess to the distance calcul

        led.set_low();

        let echo_end = SystemTimer::now();

        let echo_duration = echo_end.wrapping_sub(echo_start);

        let distance_cm = echo_duration / 16 / 58;

        // Update the distance text for the display
        if distance_cm > 1000 {
            distance_text = "I think the\nwave is lost";
            distance_text_style = base_text_style;
        } else {
            distance_text =
                format_no_std::show(&mut distance_buffer, format_args!("{} cm\n", distance_cm))
                    .unwrap();
            distance_text_style = heading_text_style;
        }

        // Draw display
        display.clear_buffer();

        center_text(&distance_text, distance_text_style)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        println!("{} cm\n", distance_cm);
    }
}

fn static_replace<T>(static_pin: &'static StaticPin<T>, pin: T) {
    critical_section::with(|cs| static_pin.borrow_ref_mut(cs).replace(pin));
}

fn trigger_ultrasonic_sensor() {
    let delay = Delay::new();
    critical_section::with(|cs| {
        let mut binding = TRIGGER.borrow_ref_mut(cs);
        let trigger = binding.as_mut().unwrap();

        trigger.set_high();
        delay.delay_micros(10);
        trigger.set_low();
    });
}

fn center_text<'a, S>(text: &'a str, style: S) -> Text<'a, S> {
    Text::with_alignment(text, Point::new(128 / 2, 64 / 2), style, Alignment::Center)
}

// Interrupt handler

#[handler]
#[ram]
fn interrupt_handler() {
    if is_interupt_source(&BUTTON) {
        println!("Button clicked");

        trigger_ultrasonic_sensor();
    }

    clear_interupt(&BUTTON);
}

fn is_interupt_source(i: &'static StaticPin<Input>) -> bool {
    critical_section::with(|cs| i.borrow_ref_mut(cs).as_mut().unwrap().is_interrupt_set())
}

fn clear_interupt(i: &'static Mutex<RefCell<Option<Input>>>) {
    critical_section::with(|cs| {
        i.borrow_ref_mut(cs).as_mut().unwrap().clear_interrupt();
    })
}
