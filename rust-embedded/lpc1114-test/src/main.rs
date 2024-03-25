#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    // SAFETY: yes... ha ha ha... YES
    let gpio0 = unsafe { &(*lpc111x_pac::Gpio0::ptr()) };

    gpio0.dir().write(|w| w.io7().bit(true));

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 12_000_000);

    loop {
        gpio0.data().write(|w| w.data7().bit(true));
        delay.delay_ms(500);
        gpio0.data().write(|w| w.data7().bit(false));
        delay.delay_ms(500);
    }
}
