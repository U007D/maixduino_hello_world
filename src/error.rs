use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{}: {:?}", "msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8", "value"))]
    ArgNotConvertibleToUtf8 {
        value: std::ffi::OsString},
}