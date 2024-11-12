use std::fmt;

/// A BIP85 error.
#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    /// Hardened index is provided, but only non-hardened indexes are allowed
    InvalidIndex(u32),
    /// Wrong number of bytes requested
    InvalidLength(u32),
    /// Wrong number of words for mnemonic
    InvalidWordCount(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidIndex(index) => write!(
                f,
                "invalid index for derivation, should be less than 0x80000000: {}",
                index,
            ),
            Error::InvalidLength(len) => write!(
                f,
                "invalid bytes length: {}. Should be between 16 and 64",
                len,
            ),
            Error::InvalidWordCount(word_count) => write!(
                f,
                "invalid number of words for mnemonic: {}. Should be 12, 18 or 24",
                word_count,
            ),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
