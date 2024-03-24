#![no_main]
#![no_std]

// Halt on panic
use panic_semihosting as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
