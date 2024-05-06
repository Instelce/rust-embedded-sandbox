#![no_std]
#![no_main]

use core::{borrow::{Borrow, BorrowMut}, cell::RefCell};

use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal as hal;
use esp_println::println;
use hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{
        self, AnyPin, Event, Floating, Input, IO
    },
    peripherals::Peripherals,
    prelude::*,
};
use log::Level;

// Rotary encoder
#[derive(PartialEq, Debug)]
enum Turn {
    Left,
    Right,
}

struct RotaryEncoder {
    clock: AnyPin<Input<Floating>>,
    data: AnyPin<Input<Floating>>,
    turn: Option<Turn>,
}

impl RotaryEncoder {
    fn new(clock: AnyPin<Input<Floating>>, data: AnyPin<Input<Floating>>) -> Self {
        Self {
            clock,
            data,
            turn: None,
        }
    }

    fn notify_turn(&mut self) {
        self.turn = if self.clock.is_high() == self.data.is_high() {
            Some(Turn::Right)
        } else {
            Some(Turn::Left)
        }
    }

    fn check_turn(&mut self) -> Option<Turn> {
        self.turn.take()
    }
}

static ENCODER: Mutex<RefCell<Option<RotaryEncoder>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let p = Peripherals::take();
    let system = p.SYSTEM.split();
    let clock = ClockControl::boot_defaults(system.clock_control).freeze();

    // configure IO
    let mut io = IO::new(p.GPIO, p.IO_MUX);
    io.set_interrupt_handler(encoder_interrupt);

    // configure rotary encoder
    let mut clk_pin = io.pins.gpio0.into_floating_input();
    let mut dt_pin = io.pins.gpio1.into_floating_input();
    // clk_pin.listen(Event::RisingEdge);
    dt_pin.listen(Event::RisingEdge);

    let encoder = RotaryEncoder::new(
        clk_pin.degrade().into(),
        dt_pin.degrade().into(),
    );

    critical_section::with(|cs| {
        *ENCODER.borrow(cs).borrow_mut() = Some(encoder);
    });

    println!("Coucou");

    loop {
        let turn = critical_section::with(|cs| {
            if let Some(encoder) = ENCODER.borrow(cs).borrow_mut().as_mut() {
                encoder.check_turn()
            } else {
                None
            }
        });

        if let Some(turn) = turn {
            if turn == Turn::Left {
                println!("Left");
            } else {
                println!("Right");
            }
        }
    }
}

#[handler]
fn encoder_interrupt() {
    println!("Interrupt");
    critical_section::with(|cs| {
        let mut encoder = ENCODER.borrow_ref_mut(cs);
        if let Some(encoder) = encoder.as_mut() {
            encoder.notify_turn();
        }
    })
}