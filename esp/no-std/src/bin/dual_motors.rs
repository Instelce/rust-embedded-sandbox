#![no_std]
#![no_main]

use core::cell::RefCell;

use critical_section::Mutex;
use critical_section::CriticalSection;
use esp_backtrace as _;
use esp_hal as hal;
use esp_println::println;
use hal::interrupt::Priority;
use hal::peripherals::Interrupt;
use hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{
        AnyPin, Event, Floating, Gpio8, Gpio9, GpioPin, Input, Output, PullDown, PullUp, PushPull,
        IO,
    },
    ledc::channel::config::PinConfig,
    peripherals::{self, Peripherals},
    prelude::*,
    riscv::{asm::nop, interrupt},
};

// Motor
struct Motor<const GPIONUM_A: u8, const GPIONUM_B: u8> {
    clockwise_pin: GpioPin<Output<PushPull>, GPIONUM_A>,
    anti_clockwise_pin: GpioPin<Output<PushPull>, GPIONUM_B>,
}

impl<const GPIONUM_A: u8, const GPIONUM_B: u8> Motor<GPIONUM_A, GPIONUM_B> {
    fn new(
        pin_a: GpioPin<Output<PushPull>, GPIONUM_A>,
        pin_b: GpioPin<Output<PushPull>, GPIONUM_B>,
    ) -> Self {
        Self {
            clockwise_pin: pin_a,
            anti_clockwise_pin: pin_b,
        }
    }
}

// static BUTTON1: Mutex<RefCell<Option<Gpio8<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
// static BUTTON2: Mutex<RefCell<Option<Gpio9<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let p = Peripherals::take();
    let system = p.SYSTEM.split();
    let clock = ClockControl::boot_defaults(system.clock_control).freeze();

    // configure IO
    let mut io = IO::new(p.GPIO, p.IO_MUX);

    // configure delay
    let delay = Delay::new(&clock);

    // configure motor 1
    let mut motor1 = Motor::new(
        io.pins.gpio6.into_push_pull_output(),
        io.pins.gpio7.into_push_pull_output(),
    );

    // configure motor 2
    let mut motor2 = Motor::new(
        io.pins.gpio4.into_push_pull_output(),
        io.pins.gpio5.into_push_pull_output(),
    );

    // configure buttons for motors
    let button1 = io.pins.gpio8.into_pull_up_input();
    let button2 = io.pins.gpio9.into_pull_up_input();

    loop {
        if button1.is_low() {
            println!("Right");
            motor1.anti_clockwise_pin.set_high();
        } else {
            motor1.anti_clockwise_pin.set_low();
        }
        if button2.is_low() {
            println!("Left");
            motor2.clockwise_pin.set_high();
        } else {
            motor2.clockwise_pin.set_low();
        }
    }
}


