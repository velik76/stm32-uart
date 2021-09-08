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
use crate::hal::{prelude::*, serial::config::Config, serial::Serial};
use core::fmt::Write; // for pretty formatting of the serial output

pub use cortex_m::*;
pub use cortex_m_rt::*;
pub use crate::hal::stm32::interrupt::*;
pub use crate::hal::stm32::*;
pub use crate::hal::*;



extern crate cortex_m_rt as rt;
extern crate nb;


/*
fn blink(led1 : stm32f4xx_hal::gpio::GPIOG::PG0<Output<PushPull>>) {
}
*/


#[entry]
fn main() -> ! {

    let delayTime = 100_u32;
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiod = dp.GPIOD.split();
    let gpiog = dp.GPIOG.split();
    let key = gpioa.pa0.into_pull_down_input();
    let mut led1 = gpiog.pg13.into_push_pull_output();
    let mut led2 = gpiog.pg14.into_push_pull_output();

    // Set up the system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    // define RX/TX pins
    let tx_pin = gpiod.pd5.into_alternate_af7();
    let rx_pin = gpioa.pa3.into_alternate_af7();
    // configure serial
    let serial = Serial::usart2(
        dp.USART2,
        (tx_pin, rx_pin),
        Config::default().baudrate(115_200.bps()),
        clocks,
    )
    .unwrap();
    let (mut tx, mut _rx) = serial.split();
    let mut value: u8 = 0;


    loop {
        led1.toggle().unwrap();
        delay.delay_ms(delayTime);

        if key.is_low().unwrap() {
            led2.set_low().unwrap();
        } else {
            led2.set_high().unwrap();
        }

        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "Hello from rust in STM32F429Discovery board. Value: {:02}\r\n", value).unwrap();
        value += 1;
    }
}
