#![no_std]
#![no_main]

use core::ptr::write_volatile;

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*, riscv::asm::nop};

#[entry]
fn main() -> ! {
    // 0x6000_4000
    const GPIO_PIN4_REG: *mut u32 = 0x6000_4084 as *mut u32;
    const DIR_OUTPUT_POS: u32 = 2;
    const PIN_DRIVE: u32 = 1 << DIR_OUTPUT_POS;

    // unsafe {
    //     write_volatile(GPIO_PIN4_REG, PIN_DRIVE);
    // }

    const GPIO_PIN4_OUT_POS: u32 = 4;
    const GPIO_OUT_REG: *mut u32 = 0x6000_4004 as *mut u32;
    let mut is_on = false;

    loop {
        unsafe {
            write_volatile(GPIO_OUT_REG, (is_on as u32) << GPIO_PIN4_OUT_POS);
        }
        for _ in 0..20_000 {
            nop();
        }
        is_on = !is_on;
    }
}
