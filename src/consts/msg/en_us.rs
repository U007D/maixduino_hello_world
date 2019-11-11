pub const ERR_NO_MCU_PERIPHERALS: &str = "Error: No microcontroller peripherals were found.";
pub const ERR_INTERNAL_INVALID_UTF8_SEQUENCE: &str =
    "Internal error: Attempted to construct an invalid UTF-8 sequence";
pub const ERR_INTERNAL_CONCAT_STR_EXACT_SIZE_MISMATCH: &str =
    "Internal error: Failed to concatenate `str slice`s into exact-sized `str` buffer due to size mismatch";
pub const ERR_INTERNAL_CONCAT_STR_EXACT_SIZE_LEN_OVF: &str =
    "Internal error: Length of concatenated `str`s exceeds maximum `str`.len() of `isize::max_value()`";
