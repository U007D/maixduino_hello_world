pub mod msg;

pub const LOOP_DELAY: usize = 500_000_000;
pub const UART_BPS_RATE: u32 = 115_200;
pub const HELLO: &str = "Hello, ";
pub const ARCH_BITS: &str = stringify!(0_usize.count_zeros());
pub const BIT_WORLD: &str = "-bit world!";
