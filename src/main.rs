#![no_main]
#![no_std]

use core::ops::BitXorAssign;
use cortex_m_rt::entry;
use lpc13xx_pac::lpc1342;
use panic_halt as _;

/// Frequency of the default IRC clock (12 Mhz)
const DEFAULT_IRC_FREQ: u32 = 12_000_000;
/// Cycles count for the stepping loop.
const LOOP_CYCLES: u32 = 5;
/// LED toggling period, ~10 seconds.
const SMALL_DELAY: u32 = DEFAULT_IRC_FREQ / LOOP_CYCLES * 10;

#[entry]
fn main() -> ! {
    let mut counter = 0u32;
    let mut led_is_on = true;

    unsafe {
        // Initialize GPIO (sets up clock)
        (*lpc1342::SYSCON::ptr())
            .sysahbclkctrl
            .write(|w| w.gpio().enabled());

        // Set GPIO1_9 pin to output.
        (*lpc1342::GPIO1::ptr()).dir.write(|w| w.io9().set_bit());

        // Initialize LED state.
        (*lpc1342::GPIO1::ptr())
            .data
            .write(|w| w.data9().variant(led_is_on));

        // Blinking loop.
        loop {
            if counter > SMALL_DELAY {
                counter = 0;
                led_is_on.bitxor_assign(true);
                (*lpc1342::GPIO1::ptr())
                    .data
                    .write(|w| w.data9().variant(led_is_on));
            } else {
                counter += 1;
                // This prevents the counter-stepping from being
                // optimized away. Overall this loop executes
                // in ~5 cycles.
                core::arch::asm!("nop");
            }
        }
    };
}
