use core::str;

use crate::{consts::msg, BoolExt, Error, Result};

pub trait ByteSliceExt {
    fn concat_exact_fit<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str>;
}

impl ByteSliceExt for [&str] {
    fn concat_exact_fit<'a>(&self, buf: &'a mut [u8]) -> Result<&'a str> {
        true.option_with(|| Some(()));
        self.iter()

            // Track the current (start) position to write into the buffer.  Because 
            // `copy_from_slice()` will panic if the src & dst byte slices are not exactly the 
            // same length, `end_pos` tracks the end-position of the upcoming copy, enabling 
            // `copy_from_slice()` to be handed exactly the right sized dst buffer.
            .fold(Ok((0_usize, 0_usize)), |pos_info, &item| {
                #[allow(unused_assignments)]

                    // Did previous iteration fail?  If not, continue concatenating src `str`s
                    pos_info.and_then(|(mut pos, end_pos)| {
                    pos = end_pos;

                    // Note: LLVM uses _signed_ pointer arithmetic (see 
                    // http://llvm.org/docs/GetElementPtr.html); the largest possible addressable
                    // structures in Rust contain `isize::max_value()` elements (not 
                    // `usize::max_value()`) (see https://github.com/rust-lang/rust/issues/18726 &
                    // https://github.com/rust-lang/unsafe-code-guidelines/issues/102).
                    // The following ensures that potentially lossy usize->isize conversions
                    // aren't lossy so that `isize`-based overflow checks are sound.
                    debug_assert!(end_pos <= isize::max_value() as usize);
                    debug_assert!(item.len() <= isize::max_value() as usize);
                    let (end_pos, overflowed) = {
                        let (i_end_pos, overflowed) = (end_pos as isize).overflowing_add(item.len
                        () as isize);
                        (i_end_pos as usize, overflowed)
                    };

                    // Will the result of the next concatenation yield a concatenation overflow?
                    // If not, proceed with concat, otherwise indicate failure with an error
                    match overflowed {
                        false => {
                            buf[pos..end_pos].copy_from_slice(item.as_bytes());
                            Ok((pos, end_pos))
                        }
                        true => Err(Error::ConcatStrExactSizeLenOverflow),
                    }
                })
            })

            // Did the concatenation operation fail?  If so, pass through the result, otherwise
            // ensure the destination buffer is not larger than the concatenated `str`
            .and_then(move |(_, concat_byte_count)| {
                (concat_byte_count == buf.len())
                    .result_with(
                        move || str::from_utf8(buf)
                            // Will not panic ∵ `buf` is comprised only of concatenated (valid UTF8)
                            //  `str`s
                            .unwrap_or_else(|_| {
                                unreachable!(msg::ERR_INTERNAL_INVALID_UTF8_SEQUENCE)
                            }),
                        || Error::ConcatStrExactSizeMismatch)
            })
    }
}
