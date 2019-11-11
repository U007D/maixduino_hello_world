use crate::consts::msg;
use core::str::Utf8Error;
use core::{
    fmt::{Display, Error as FmtError, Formatter},
    str,
};

#[derive(Clone, Debug)]
pub enum Error {
    ConcatStrExactSizeLenOverflow,
    ConcatStrExactSizeMismatch,
    NoMcuPeripherals,
    StdFmtError(FmtError),
    Utf8Error(str::Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Error::NoMcuPeripherals => write!(f, "{}", msg::ERR_NO_MCU_PERIPHERALS),
            Error::StdFmtError(err) => err.fmt(f),
            Error::Utf8Error(err) => err.fmt(f),
            Error::ConcatStrExactSizeMismatch => {
                write!(f, "{}", msg::ERR_INTERNAL_CONCAT_STR_EXACT_SIZE_MISMATCH)
            }
            Error::ConcatStrExactSizeLenOverflow => {
                write!(f, "{}", msg::ERR_INTERNAL_CONCAT_STR_EXACT_SIZE_LEN_OVF)
            }
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

impl From<FmtError> for Error {
    fn from(err: FmtError) -> Self {
        Self::StdFmtError(err)
    }
}
