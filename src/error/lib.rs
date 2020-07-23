use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::fmt::{self, Display, Debug, Formatter};

pub struct Error(Box<ErrorInner>);

struct ErrorInner {
    kind: Box<dyn Display + Send + Sync + 'static>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl Error {
    pub fn new<K>(kind: K) -> Error
    where K: Display + Send + Sync + 'static
    {
        Error(Box::new(ErrorInner {
            kind: Box::new(kind),
            source: None,
        }))
    }

    pub fn from_source<E, K>(source: E, kind: K) -> Error
    where E: StdError + Send + Sync + 'static,
            K: Display + Send + Sync + 'static
    {
        Error(Box::new(ErrorInner {
            kind: Box::new(kind),
            source: Some(Box::new(source)),
        }))
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.0.source {
            Some(ref e) => Some(e.as_ref()),
            None => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.kind.fmt(f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("YASP Error")
            .field("kind", &format!("{}", &self.0.kind))
            .field("source", &self.0.source)
            .finish()
    }
}

pub type Result<T> = StdResult<T, Error>;
