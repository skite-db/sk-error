//! This module provide a wrapper for the message in [Error][SkError].

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorMessage<'a> {
    New { msg: &'a str },
}

impl<'a> ErrorMessage<'a> {
    /// Create a new error message.
    #[inline(always)]
    pub const fn new(msg: &'a str) -> Self {
        Self::New { msg }
    }

    ///
    #[inline(always)]
    pub const fn as_str(&self) -> &'a str {
        match self {
            Self::New { msg } => msg,
        }
    }
}

impl<'a> core::fmt::Display for ErrorMessage<'a> {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        fmt.write_str(self.as_str())
    }
}

impl<'a> PartialEq<&str> for ErrorMessage<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> From<&'a str> for ErrorMessage<'a> {
    #[inline(always)]
    fn from(msg: &'a str) -> Self {
        Self::new(msg)
    }
}

impl<'a> AsRef<str> for ErrorMessage<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> core::error::Error for ErrorMessage<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message_display() {
        let err = ErrorMessage::new("DATABASE_LOCKED");

        assert_eq!(format!("{}", err), "DATABASE_LOCKED");

        assert_eq!(err, "DATABASE_LOCKED");

        println!("\n[Test Log] Error: {}", err);
    
    }
}
