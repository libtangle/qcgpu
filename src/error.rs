use failure::{Backtrace, Context, Fail};
use std::fmt;

#[derive(Debug)]
struct BackendError {
    inner: Context<BackendErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
enum BackendErrorKind {
    #[fail(display = "System doesn't have enough memory.")]
    MemoryError,
}

impl Fail for BackendError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl BackendError {
    pub fn kind(&self) -> BackendErrorKind {
        *self.inner.get_context()
    }
}

impl From<BackendErrorKind> for BackendError {
    fn from(kind: BackendErrorKind) -> BackendError {
        BackendError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<BackendErrorKind>> for BackendError {
    fn from(inner: Context<BackendErrorKind>) -> BackendError {
        BackendError { inner: inner }
    }
}
