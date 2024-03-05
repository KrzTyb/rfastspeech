/// Toolkit error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// std::io::Error
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// With path
    #[error("{inner}; path: {path:?}")]
    WithPath {
        inner: Box<Self>,
        path: std::path::PathBuf,
    },

    /// With backtrace
    #[error("{inner}\n{backtrace}")]
    WithBacktrace {
        inner: Box<Self>,
        backtrace: Box<std::backtrace::Backtrace>,
    },

    /// Generic error
    #[error("{0}")]
    Message(String),
}

impl Error {
    pub fn message<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Message(err.to_string())
    }

    pub fn add_path<P>(self, p: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        Self::WithPath {
            inner: Box::new(self),
            path: p.as_ref().to_path_buf(),
        }
    }

    pub fn add_backtrace(self) -> Self {
        let bt = std::backtrace::Backtrace::capture();
        match bt.status() {
            std::backtrace::BacktraceStatus::Disabled
            | std::backtrace::BacktraceStatus::Unsupported => self,
            _ => Self::WithBacktrace {
                inner: Box::new(self),
                backtrace: Box::new(bt),
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Return with an error message
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err($crate::Error::Message(format!($msg).into()).add_backtrace())
    };
    ($err:expr $(,)?) => {
        return Err($crate::Error::Message(format!($err).into()).add_backtrace())
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::Error::Message(format!($fmt, $($arg)*).into()).add_backtrace())
    };
}
