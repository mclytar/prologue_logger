//! Errors returned by bad configuration of log entries or the logger itself.

use std::fmt::{Formatter};

use crate::{Entry, EntrySourceBuilder, MultiEntry, NoStyler, Styler};

/// Alias type for `std::result::Result<T, prologue_emitter::error::Error>`
pub type Result<T, STYLER = NoStyler> = std::result::Result<T, Error<STYLER>>;

/// Contains the `struct` which was being constructed before the `Error` happened, if any.
#[derive(Debug)]
pub enum PartialConfiguration<STYLER: Styler = NoStyler> {
    /// The partial configuration is empty.
    None,
    /// The partial configuration contains an [`Entry`](super::Entry).
    Entry(super::Entry<STYLER>),
    /// The partial configuration contains an [`EntrySourceBuilder`](super::EntrySourceBuilder).
    EntrySourceBuilder(super::EntrySourceBuilder<STYLER>),
    /// The partial configuration contains a [`MultiEntry`](super::MultiEntry).
    MultiEntry(super::MultiEntry)
}
impl<STYLER: Styler> From<super::Entry<STYLER>> for PartialConfiguration<STYLER> {
    fn from(entry: Entry<STYLER>) -> Self {
        PartialConfiguration::Entry(entry)
    }
}
impl<STYLER: Styler> From<super::EntrySourceBuilder<STYLER>> for PartialConfiguration<STYLER> {
    fn from(esb: EntrySourceBuilder<STYLER>) -> Self {
        PartialConfiguration::EntrySourceBuilder(esb)
    }
}
impl From<super::MultiEntry> for PartialConfiguration {
    fn from(me: MultiEntry) -> Self {
        PartialConfiguration::MultiEntry(me)
    }
}

/// Error `struct` handling the various errors which may arise during the construction
/// of a log entry.
///
/// It contains the [`ErrorKind`] describing the type of error, and a [`PartialConfiguration`]
/// containing any `struct` which was being built before the error happened, if any.
#[derive(Debug)]
pub struct Error<STYLER: Styler = NoStyler> {
    kind: ErrorKind,
    partial_configuration: PartialConfiguration<STYLER>
}
impl<STYLER: Styler> std::fmt::Display for Error<STYLER> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (&self.kind as &dyn std::fmt::Display).fmt(f)
    }
}
impl<STYLER: Styler> From<ErrorKind> for Error<STYLER> {
    fn from(kind: ErrorKind) -> Self {
        Error { kind, partial_configuration: PartialConfiguration::None }
    }
}
#[cfg(feature = "log")]
impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error { kind: ErrorKind::SetLoggerError(err), partial_configuration: PartialConfiguration::None }
    }
}
#[cfg(feature = "indicatif")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error { kind: ErrorKind::IoError(Box::new(err)), partial_configuration: PartialConfiguration::None }
    }
}
impl<STYLER: Styler> std::error::Error for Error<STYLER> {}
impl<STYLER: Styler> Error<STYLER> {
    /// Returns the type of error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Attaches a partial configuration to this `struct`.
    pub(crate) fn set_partial_configuration<P: Into<PartialConfiguration<STYLER>>>(&mut self, cfg: P) {
        self.partial_configuration = cfg.into();
    }

    /// Discards the error and returns the inner partial configuration.
    pub fn into_partial_configuration(self) -> PartialConfiguration<STYLER> {
        let Error { partial_configuration: partial, .. } = self;
        partial
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
impl ErrorKind {
    /// Promotes this `ErrorKind` to an error with the given partial configuration.
    pub(crate) fn into_error_with_partial_configuration<STYLER: Styler, P: Into<PartialConfiguration<STYLER>>>(self, cfg: P) -> Error<STYLER> {
        let partial = cfg.into();
        Error { kind: self, partial_configuration: partial }
    }
}


#[cfg(feature = "log")]
impl From<log::SetLoggerError> for ErrorKind {
    fn from(err: log::SetLoggerError) -> Self {
        ErrorKind::SetLoggerError(err)
    }
}