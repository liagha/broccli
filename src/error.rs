pub enum Error {
    Io(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}

impl core::fmt::Debug for Error {
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}
