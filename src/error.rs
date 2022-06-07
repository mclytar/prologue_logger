//! Errors returned by bad configuration of log entries or the logger itself.

use std::fmt::{Formatter};

use crate::{StdWriter, StyledWriter};

/// Alias type for `std::result::Result<T, prologue_emitter::error::Error>`
pub type Result<T> = std::result::Result<T, Error>;

/*/// Contains the `struct` which was being constructed before the `Error` happened, if any.
#[derive(Debug)]
pub enum PartialConfiguration {
    /// The partial configuration is empty.
    None,
    /// The partial configuration contains an [`Entry`](super::Entry).
    Entry(super::Entry),
    /// The partial configuration contains an [`EntrySourceBuilder`](super::EntrySourceBuilder).
    EntrySourceBuilder(super::EntrySourceBuilder),
    /// The partial configuration contains a [`MultiEntry`](super::MultiEntry).
    MultiEntry(super::MultiEntry)
}
impl From<super::Entry> for PartialConfiguration {
    fn from(entry: Entry) -> Self {
        PartialConfiguration::Entry(entry)
    }
}
impl From<super::EntrySourceBuilder> for PartialConfiguration {
    fn from(esb: EntrySourceBuilder) -> Self {
        PartialConfiguration::EntrySourceBuilder(esb)
    }
}
impl From<super::MultiEntry> for PartialConfiguration {
    fn from(me: MultiEntry) -> Self {
        PartialConfiguration::MultiEntry(me)
    }
}*/

/// Error `struct` handling the various errors which may arise during the construction
/// of a log entry.
///
/// It contains the [`ErrorKind`] describing the type of error, and a [`PartialConfiguration`]
/// containing any `struct` which was being built before the error happened, if any.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    //partial_configuration: PartialConfiguration
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (&self.kind as &dyn std::fmt::Display).fmt(f)
    }
}
impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error { kind }
    }
}
#[cfg(feature = "log")]
impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error { kind: ErrorKind::SetLoggerError(err) }
    }
}
#[cfg(feature = "indicatif")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error { kind: ErrorKind::IoError(Box::new(err)) }
    }
}
impl std::error::Error for Error {}
impl Error {
    /// Returns the type of error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

/// Enumerator describing the type of error.
#[derive(Debug)]
pub enum ErrorKind {
    /// The given target already exists.
    TargetAlreadyExists(String),
    /// There was an attempt to annotate an empty line.
    AnnotationOnEmptyLine,
    /// There was an attempt to overlap annotations on a source code line.
    OverlappingAnnotation,
    /// There was an attempt to set multiple loggers.
    ///
    /// For further information, see crate [`log`](https://docs.rs/log/0.4.17/log/index.html).
    #[cfg(feature = "log")]
    SetLoggerError(log::SetLoggerError),
    /// Generic IO error.
    #[cfg(feature = "indicatif")]
    IoError(Box<std::io::Error>)
}
impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::TargetAlreadyExists(target) => write!(f, "target `{}` already exists", target),
            ErrorKind::AnnotationOnEmptyLine => write!(f, "tried to annotate an empty line"),
            ErrorKind::OverlappingAnnotation => write!(f, "annotation overlaps with previous annotation"),
            #[cfg(feature = "log")]
            ErrorKind::SetLoggerError(err) => (err as &dyn std::fmt::Display).fmt(f),
            #[cfg(feature = "indicatif")]
            ErrorKind::IoError(err) => (err as &dyn std::fmt::Display).fmt(f)
        }
    }
}

#[cfg(feature = "log")]
impl From<log::SetLoggerError> for ErrorKind {
    fn from(err: log::SetLoggerError) -> Self {
        ErrorKind::SetLoggerError(err)
    }
}