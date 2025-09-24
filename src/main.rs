#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit as _;
use microbit::board::Board;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print, rdbg};

use nrf52833_hal::{pac, timer, gpio};
use embedded_hal::{delay::DelayNs};

#[entry]
fn main() -> ! {

    
    // let mut board = Board::take().unwrap();
    // board.display_pins.col2.set_low().unwrap();
    // board.display_pins.row2.set_high().unwrap();

    // Access Peripherals Access Crate (PAC)
    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let _row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    #[allow(clippy::empty_loop)]
    loop {
        asm::nop();

    }
}
