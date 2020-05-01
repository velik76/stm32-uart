#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![allow(non_snake_case)]
// #![allow(non_camel_case_types)]


// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};


#[entry]
fn main() -> ! {

    let delayTime = 20_u32;
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiog = dp.GPIOG.split();
    let key = gpioa.pa0.into_pull_down_input();
    let mut led1 = gpiog.pg13.into_push_pull_output();
    let mut led2 = gpiog.pg14.into_push_pull_output();

    // Set up the system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    loop {
        led1.set_high().unwrap();
        delay.delay_ms(delayTime);
        led1.set_low().unwrap();
        delay.delay_ms(delayTime);

        if key.is_low().unwrap() {
            led2.set_low().unwrap();
        } else {
            led2.set_high().unwrap();
        }
    }
}
