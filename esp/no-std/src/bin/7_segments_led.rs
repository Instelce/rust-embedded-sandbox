#![no_std]
#![no_main]

use core::cell::RefCell;

use critical_section::Mutex;
use embedded_hal::digital::OutputPin;
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::{Event, GpioPin, Input, Output, PullDown, PushPull, IO}, peripherals::Peripherals, prelude::*, riscv::asm::nop};
use esp_println::println;

trait Use {
    type Error;

    fn zero(&mut self) -> Result<(), Self::Error>;
    fn one(&mut self) -> Result<(), Self::Error>;
    fn two(&mut self) -> Result<(), Self::Error>;
    fn three(&mut self) -> Result<(), Self::Error>;
    fn four(&mut self) -> Result<(), Self::Error>;
    fn five(&mut self) -> Result<(), Self::Error>;
    fn six(&mut self) -> Result<(), Self::Error>;
    fn seven(&mut self) -> Result<(), Self::Error>;
    fn height(&mut self) -> Result<(), Self::Error>;
    fn nine(&mut self) -> Result<(), Self::Error>;

    fn dot(&mut self) -> Result<(), Self::Error>;
    fn reset(&mut self) -> Result<(), Self::Error>;

    fn display(&mut self, number: u8) -> Result<(), Self::Error> {
        match number {
            0 => self.zero(),
            1 => self.one(),
            2 => self.two(),
            3 => self.three(),
            4 => self.four(),
            5 => self.five(),
            6 => self.six(),
            7 => self.seven(),
            8 => self.height(),
            9 => self.nine(),
            _ => self.reset()
        }
    }
}
struct SevenSegmentsLed<A, B, C, D, E, F, G, DP, OutputPinError>
where
    A: OutputPin<Error = OutputPinError>,
    B: OutputPin<Error = OutputPinError>,
    C: OutputPin<Error = OutputPinError>,
    D: OutputPin<Error = OutputPinError>,
    E: OutputPin<Error = OutputPinError>,
    F: OutputPin<Error = OutputPinError>,
    G: OutputPin<Error = OutputPinError>,
    DP: OutputPin<Error = OutputPinError>
{
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    dp: DP
}

impl<A, B, C, D, E, F, G, DP, OutputPinError> SevenSegmentsLed<A, B, C, D, E, F, G, DP, OutputPinError>
where
    A: OutputPin<Error = OutputPinError>,
    B: OutputPin<Error = OutputPinError>,
    C: OutputPin<Error = OutputPinError>,
    D: OutputPin<Error = OutputPinError>,
    E: OutputPin<Error = OutputPinError>,
    F: OutputPin<Error = OutputPinError>,
    G: OutputPin<Error = OutputPinError>,
    DP: OutputPin<Error = OutputPinError>
{
    fn new(a: A, b: B, c: C, d: D, e: E, f: F, g: G, dp: DP) -> Self {
        Self {
            a, b, c, d, e, f, g, dp
        }
    }
}

impl<A, B, C, D, E, F, G, DP, OutputPinError> Use for SevenSegmentsLed<A, B, C, D, E, F, G, DP, OutputPinError>
where
    A: OutputPin<Error = OutputPinError>,
    B: OutputPin<Error = OutputPinError>,
    C: OutputPin<Error = OutputPinError>,
    D: OutputPin<Error = OutputPinError>,
    E: OutputPin<Error = OutputPinError>,
    F: OutputPin<Error = OutputPinError>,
    G: OutputPin<Error = OutputPinError>,
    DP: OutputPin<Error = OutputPinError>
{
    type Error = OutputPinError;

    fn zero(&mut self) -> Result<(), Self::Error> {
        self.a.set_high()?;
        self.b.set_high()?;
        self.c.set_high()?;
        self.d.set_high()?;
        self.e.set_high()?;
        self.f.set_high()
    }

    fn one(&mut self) -> Result<(), Self::Error> {
        self.b.set_high()?;
        self.c.set_high()
    }

    fn two(&mut self) -> Result<(), Self::Error> {
        self.a.set_high()?;
        self.b.set_high()?;
        self.g.set_high()?;
        self.e.set_high()?;
        self.d.set_high()
    }

    fn three(&mut self) -> Result<(), Self::Error> {
        self.a.set_high()?;
        self.b.set_high()?;
        self.g.set_high()?;
        self.c.set_high()?;
        self.d.set_high()
    }

    fn four(&mut self) -> Result<(), Self::Error> {
        self.f.set_high()?;
        self.g.set_high()?;
        self.b.set_high()?;
        self.c.set_high()
    }

    fn five(&mut self) -> Result<(), Self::Error> {
        self.a.set_high()?;
        self.f.set_high()?;
        self.g.set_high()?;
        self.c.set_high()?;
        self.d.set_high()
    }

    fn six(&mut self) -> Result<(), Self::Error> {
        self.a.set_high()?;
        self.f.set_high()?;
        self.g.set_high()?;
        self.e.set_high()?;
        self.c.set_high()?;
        self.d.set_high()
    }

    fn seven(&mut self) -> Result<(), Self::Error> {
        self.a.set_high()?;
        self.b.set_high()?;
        self.c.set_high()
    }

    fn height(&mut self) -> Result<(), Self::Error> {
        self.a.set_high()?;
        self.b.set_high()?;
        self.c.set_high()?;
        self.d.set_high()?;
        self.e.set_high()?;
        self.f.set_high()?;
        self.g.set_high()
    }

    fn nine(&mut self) -> Result<(), Self::Error> {
        self.a.set_high()?;
        self.b.set_high()?;
        self.f.set_high()?;
        self.g.set_high()?;
        self.c.set_high()?;
        self.d.set_high()
    }

    fn dot(&mut self) -> Result<(), Self::Error> {
        self.dp.set_high()
    }

    fn reset(&mut self) -> Result<(), Self::Error> {
        self.a.set_low()?;
        self.b.set_low()?;
        self.c.set_low()?;
        self.d.set_low()?;
        self.e.set_low()?;
        self.f.set_low()?;
        self.g.set_low()?;
        self.dp.set_low()
    }
}

static INCREASE_BUTTON: Mutex<RefCell<Option<GpioPin<Input<PullDown>, 8>>>> = Mutex::new(RefCell::new(None));
static COUNTER: Mutex<RefCell<u8>> = Mutex::new(RefCell::new(0));

#[entry]
fn main() -> ! {
    let p = Peripherals::take();
    let system = p.SYSTEM.split();
    let clock = ClockControl::boot_defaults(system.clock_control).freeze();

    // configure IO
    let mut io = IO::new(p.GPIO, p.IO_MUX);
    io.set_interrupt_handler(increase_handler);

    // configure delay
    let delay = Delay::new(&clock);

    // configure 7 segments LED
    let mut seven_segments = SevenSegmentsLed::new(
        io.pins.gpio0.into_push_pull_output(),
        io.pins.gpio1.into_push_pull_output(),
        io.pins.gpio2.into_push_pull_output(),
        io.pins.gpio3.into_push_pull_output(),
        io.pins.gpio4.into_push_pull_output(),
        io.pins.gpio5.into_push_pull_output(),
        io.pins.gpio6.into_push_pull_output(),
        io.pins.gpio7.into_push_pull_output()
    );

    // configure buttons to pins 8 and 9
    let mut increase_button = io.pins.gpio8.into_pull_down_input();
    let reset_button = io.pins.gpio9.into_pull_down_input();

    increase_button.listen(Event::FallingEdge);

    critical_section::with(|cs| {
        INCREASE_BUTTON.borrow_ref_mut(cs).replace(increase_button);
    });

    println!("Coucou");

    loop {
        // get counter value
        let counter = critical_section::with(|cs| {
            *COUNTER.borrow(cs).borrow()
        });
        println!("Counter: {}", counter);

        // display counter
        seven_segments.reset().unwrap();
        seven_segments.display(counter).unwrap();

        // check reset button
        if reset_button.is_low() {
            println!("Reset");
            critical_section::with(|cs| {
                *COUNTER.borrow(cs).borrow_mut() = 0;
            });
        }
    }
}

#[handler]
fn increase_handler() {
    critical_section::with(|cs| {
        if let Some(increate_button) = INCREASE_BUTTON.borrow(cs).borrow_mut().as_mut() {
            println!("Interrupt {}", critical_section::with(|cs| *COUNTER.borrow(cs).borrow()));
            increate_button.clear_interrupt();

            if *COUNTER.borrow(cs).borrow() < 9 {
                *COUNTER.borrow(cs).borrow_mut() += 1;
            }
        }
    });
}