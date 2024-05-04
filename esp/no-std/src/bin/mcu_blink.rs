#![no_std]
#![no_main]

use core::ptr::write_volatile;

use esp_backtrace as _;
use esp_hal::{prelude::*, riscv::asm::nop};

#[entry]
fn main() -> ! {
    const GPIO_BASE_ADDR: u32 = 0x6000_4000 as u32;
    const GPIO_BIT_SELECT_REG: u32 = GPIO_BASE_ADDR;
    const GPIO_OUT_REG: *mut u32 = (GPIO_BASE_ADDR + 0x0004) as *mut u32;
    const GPIO_PIN_BASE_ADDR: u32 = GPIO_BASE_ADDR + 0x0074;
    const GPIO_PIN4_REG: *mut u32 = (GPIO_PIN_BASE_ADDR + 4 * 4) as *mut u32;
    const DIR_OUTPUT_POS: u32 = 4;
    const PIN_DRIVE: u32 = 1 << DIR_OUTPUT_POS;

    unsafe {
        write_volatile(GPIO_PIN4_REG, PIN_DRIVE);
    }

    const GPIO_PIN4_OUT_POS: u32 = 4;
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
