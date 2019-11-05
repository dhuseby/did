use failure::{Backtrace, Context, Fail};

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum DidErrorKind {
    #[fail(display = "Invalid did uri specified. Must start with 'did:'")]
    InvalidUri,
    #[fail(display = "Unknown did method: {:?}", msg)]
    UnknownMethod { msg: String },
}

#[derive(Debug)]
pub struct DidError {
    inner: Context<DidErrorKind>,
}

impl Fail for DidError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl DidError {
    pub fn from_msg<D>(kind: DidErrorKind, msg: D) -> DidError
    where
        D: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
    {
        DidError {
            inner: Context::new(msg).context(kind),
        }
    }

    pub fn from_kind(kind: DidErrorKind) -> DidError {
        DidError {
            inner: Context::new("").context(kind),
        }
    }

    pub fn kind(&self) -> DidErrorKind {
        self.inner.get_context().clone()
    }
}

impl std::fmt::Display for DidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;

        for cause in Fail::iter_chain(&self.inner) {
            if first {
                first = false;
                writeln!(f, "Error: {}", cause)?;
            } else {
                writeln!(f, "Caused by: {}", cause)?;
            }
        }

        Ok(())
    }
}

pub fn err_msg<D>(kind: DidErrorKind, msg: D) -> DidError
where
    D: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
{
    DidError::from_msg(kind, msg)
}

impl From<Context<DidErrorKind>> for DidError {
    fn from(inner: Context<DidErrorKind>) -> DidError {
        DidError { inner }
    }
}

/// Extension methods for `Error`.
pub trait DidErrorExt {
    fn to_did<D>(self, kind: DidErrorKind, msg: D) -> DidError
    where
        D: std::fmt::Display + Send + Sync + 'static;
}

impl<E> DidErrorExt for E
where
    E: Fail,
{
    fn to_did<D>(self, kind: DidErrorKind, msg: D) -> DidError
    where
        D: std::fmt::Display + Send + Sync + 'static,
    {
        self.context(msg).context(kind).into()
    }
}
