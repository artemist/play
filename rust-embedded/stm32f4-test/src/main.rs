#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{gpio::PinState, pac, prelude::*};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let gpiod = p.GPIOD.split();
    let mut leds = [
        gpiod
            .pd12
            .into_push_pull_output_in_state(PinState::Low)
            .erase(),
        gpiod
            .pd13
            .into_push_pull_output_in_state(PinState::Low)
            .erase(),
        gpiod
            .pd14
            .into_push_pull_output_in_state(PinState::Low)
            .erase(),
        gpiod
            .pd15
            .into_push_pull_output_in_state(PinState::Low)
            .erase(),
    ];
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        for led in leds.iter_mut() {
            led.toggle();
            delay.delay_ms(250);
        }
    }
}
