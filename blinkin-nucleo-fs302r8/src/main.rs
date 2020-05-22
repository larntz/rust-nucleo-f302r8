#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m;
use cortex_m_rt::entry;

use stm32f3xx_hal as hal;
use crate::hal::{prelude::*, stm32};
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let sp = stm32::Peripherals::take().unwrap();
    let mut rcc = sp.RCC.constrain();
    let mut gpiob = sp.GPIOB.split(&mut rcc.ahb);
    let mut led = gpiob.pb13.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
   
    let mut flash = sp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    led.set_high().unwrap();
    // let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    loop {
        led.set_low().unwrap();
        delay.delay_ms(2000_u32);
        led.set_high().unwrap();
        delay.delay_ms(2000_u32);
    }
}
