use crate::ByteSliceExt;
use crate::{consts::*, Error, Result};
use k210_hal::{pac, prelude::*, stdout::Stdout};

pub fn run() -> Result<()> {
    let p = pac::Peripherals::take().ok_or_else(|| Error::NoMcuPeripherals)?;

    // Configure clock
    let clock = k210_hal::clock::Clocks::new();

    // Configure UART
    let serial = p.UARTHS.configure(UART_BPS_RATE.bps(), &clock);
    let (mut tx, _) = serial.split();

    // Prepare greeting
    let mut buf = [0_u8; HELLO.len() + ARCH_BITS.len() + BIT_WORLD.len()];
    let greeting = [HELLO, ARCH_BITS, BIT_WORLD].concat_exact_fit(&mut buf)?;

    // Write output
    let mut stdout = Stdout(&mut tx);
    writeln!(stdout, "{}", greeting)?;

    Ok(())
}
