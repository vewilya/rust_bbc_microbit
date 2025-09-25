#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm;

use microbit as _;
// use microbit::board::Board;
use panic_halt as _;
// use rtt_target::{rdbg, rprintln, rtt_init_print};

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::hal::{gpio, timer};

#[entry]
fn main() -> ! {

    let board = microbit::Board::take().unwrap();
    let button = board.buttons.button_a;
    let mut row1 = board.display_pins.row1.into_push_pull_output(gpio::Level::High);
    let col1 = board.display_pins.col2.into_push_pull_output(gpio::Level::Low);

    let mut timer0 = timer::Timer::new(board.TIMER0); 

    #[allow(clippy::empty_loop)]
    loop {
        asm::nop();

        timer0.delay_ms(500);
        row1.set_high().unwrap();
        timer0.delay_ms(500);
        row1.set_low().unwrap();
    }
}
