use core::fmt;

/// Hex encoding / decoding error
#[derive(PartialEq)]
pub enum Error {
    BufferOverflow,
    InvalidEncoding,
    InvalidLength,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BufferOverflow => {
                write!(f, "{}", "insufficient buffer length")
            }
            Self::InvalidEncoding => {
                write!(f, "{}", "invalid hex encoding")
            }
            Self::InvalidLength => {
                write!(f, "{}", "invalid length of source")
            }
        }
    }
}
