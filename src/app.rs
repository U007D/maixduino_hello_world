use crate::{consts::*, Error, Result};
use core::str;
use k210_hal::{pac, prelude::*, stdout::Stdout};

pub fn run() -> Result<()> {
    let p = pac::Peripherals::take().ok_or_else(|| Error::NoMcuPeripherals)?;

    // Configure clock
    let clock = k210_hal::clock::Clocks::new();

    // Configure UART
    let serial = p.UARTHS.configure(UART_BPS_RATE.bps(), &clock);
    let (mut tx, _) = serial.split();

    // Prep greeting
    let mut greeting_builder = [0_u8; HELLO.len() + ARCH_BITS.len() + BIT_WORLD.len()];
    greeting_builder.copy_from_slice(HELLO.as_bytes());
    greeting_builder[HELLO.len()..].copy_from_slice(ARCH_BITS.as_bytes());
    greeting_builder[HELLO.len() + ARCH_BITS.len()..].copy_from_slice(BIT_WORLD.as_bytes());
    let greeting = str::from_utf8(&greeting_builder)?;

    // Write output
    let mut stdout = Stdout(&mut tx);
    writeln!(stdout, "{}", greeting)?;

    Ok(())
}
