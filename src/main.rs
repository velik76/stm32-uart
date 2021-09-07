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

use crate::hal::{prelude::*, stm32, serial::config::Config, serial::Serial};

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

//    let usart1 = dp.USART1;
    let cfg = serial::config::Config::default().baudrate(115_200.bps());
    let usart2 = dp.USART2;
    let tx_pin = gpioa.pa2.into_alternate_af0();
    // configure serial
    let mut tx = Serial::tx(
        dp.USART2,
        tx_pin,
        Config::default().baudrate(9600.bps()),
        clocks,
    )
    .unwrap();
    

//    let mut usart2 = dp.USART2.usart(gpioa.pa2, gpioa.pa3, cfg, &mut rcc).unwrap();

    loop {
//        blink(led1);

        led1.toggle().unwrap();
//        led1.set_high().unwrap();
        delay.delay_ms(delayTime);
//        led1.set_low().unwrap();
//        delay.delay_ms(delayTime);

        if key.is_low().unwrap() {
            led2.set_low().unwrap();
//            let _ = led2.set_low();
        } else {
            led2.set_high().unwrap();
        }
//        cortex_m::asm::wfi();
    }
}
