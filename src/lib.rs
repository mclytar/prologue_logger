//! A Rust library to produce Rust-like logs for source code or settings files.
//!
//! It offers many features, like:
//! * easy-to-use building patterns to customize the log entries;
//! * annotating source code lines with the Rust-like `^^^` underline;
//! * counting warnings and errors for multiple targets;
//! * colored output to `stderr` (requires the `console` feature);
//! * integration with the [`log`](https://docs.rs/log/latest/log/) API (requires the `log` feature);
//! * integration with the [`indicatif`](https://docs.rs/indicatif/latest/indicatif/) crate (requires the `indicatif` feature);
//! * color support with the [`console`](https://docs.rs/console/latest/console/) crate (requires the `console` feature).
//!
//! # Usage
//!
//! The simplest usage consists of creating a target to log to and then log entries to it.
//!
//! ## Example
//! ```
//! # use prologue_logger::{Target, Entry, Task};
//! // Create a target.
//! let target = Target::new("my-target");
//! // Log to the above target.
//! Task::new("Doing", "some work on `my-target`")
//!     .log_to_target(&target);
//! Entry::new_warning("this is a warning")
//!     .log_to_target(&target);
//! Entry::new_note("some other information about the above warning")
//!     .log_to_target(&target);
//! Task::new("Finish", "job on `my-target`")
//!     .log_to_target(&target);
//! ```
//! The above code will produce the following result:
//! ```text
//!        Doing some work on `my-target`
//! warning: this is a warning
//! note: some other information about the above warning
//!       Finish job on `my-target`
//! ```
//!
//! ## Entry customization
//!
//! It is possible to highly customize the log entry by specifying what caused it.
//! To see another example, let's try to recreate Rust output for the following code
//! in a file called `src/main.rs`.
//! ```rust,ignore
//! #![allow(unused_variables)]
//! fn main() {
//!     let mut x = 42;
//! }
//! ```
//! The Rust compiler would complain that the variable `x` does not need to be mutable;
//! let's do the same.
//! ```
//! // Example available at `examples/doc_entry_customization.rs`.
//! // Use the feature `console` to emit a colored output.
//!
//! # fn main() -> prologue_logger::error::Result<()> {
//! # use prologue_logger::{Entry, Target, Task};
//! // Target receiving global messages.
//! // Note: "" is just a name and has no particular meaning.
//! let global_target = Target::new("");
//! // Target receiving messages for `example`.
//! let target = Target::new("example");
//!
//! // First, log the fact that we are compiling our example.
//! Task::new("Compiling", "example v0.1.0")
//!     .log_to_target(&target);
//! // Then, emit a warning for the mutable `x`.
//! Entry::new_warning("variable does not need to be mutable")
//!     // The warning is for file `src/main.rs`, line 3, position 9.
//!     .named_source("src/main.rs", 3, 9)
//!     // Set the source line number (i.e., 3) and the respective source code.
//!     .new_line(3, "    let mut x = 42;")
//!     // Annotate the `mut ` string and add an help string.
//!     .annotate_help(9, 4, "help: remove this `mut`")?
//!     // Annotate the variable `x` (no help string needed).
//!     .annotate_warn(13, 1, "")?
//!     // Add a final note explaining why the user sees this warning.
//!     .note("`#[warn(unused_mut)]` on by default")
//!     // Finish the log line and output it.
//!     .finish()
//!     .log_to_target(&target)?;
//! // Then, finish parsing the file `src/main.rs` and output a summary of the warnings.
//! target.if_errors(|count|
//!     // This will not be displayed because no errors happened in `target`.
//!     Entry::new_error(format!("Could not run `example` due to {} previous error{}", count, if count > 1 { "s" } else { "" }))
//!         .log_to_target(&global_target));
//! target.if_warnings(|count|
//!     // This will be displayed because at least one warning was generated in `target`.
//!     Entry::new_warning(format!("`example` (bin) generated {} warning{}", count, if count > 1 { "s" } else { "" }))
//!         // However, we don't want to add the "summary warning" to the warning count for `example`,
//!         // therefore we log it to the global target.
//!         .log_to_target(&global_target));
//! // Finally, emit some entry telling that the compilation is finished.
//! Task::new("Finished", "dev [unoptimized + debuginfo] target(s)")
//!     .log_to_target(&global_target);
//! # Ok(()) }
//! ```
//! The above code should produce the following output:
//! ```text
//!       Compiling example v0.1.0
//! warning: variable does not need to be mutable
//!  --> src/main.rs:3:9
//!   |
//! 3 |     let mut x = 42;
//!   |         ----^
//!   |         |
//!   |         help: remove this `mut`
//!   |
//!   = note: `#[warn(unused_mut)]` on by default
//!
//! warning: `example` (bin) generated 1 warning
//!     Finished dev [unoptimized + debuginfo] target(s)
//! ```
//!
//! # Multiple entries
//!
//! Sometimes, log events are made of multiple, related entries.
//! Consider the following code.
//! ```
//! // Example available at `examples/doc_multi_entry_first.rs`.
//! // Use the feature `console` to emit a colored output.
//!
//! # fn main() -> prologue_logger::error::Result<()> {
//! # use prologue_logger::{Entry, Target, Task};
//! // Target `example`
//! let target = Target::new("example");
//!
//! // Emit a warning for missing documentation.
//! Entry::new_warning("missing documentation for an associated function")
//!     .named_source("src/lib.rs", 1163, 5)
//!     .new_line(1163, "    pub fn log_to_target(self, target: &Target) {")
//!     .annotate_warn(5, 43, "")?
//!     .finish()
//!     .log_to_target(&target)?;
//! // Then, emit a note about the lint level.
//! Entry::new_note("the lint level is defined here")
//!     .named_source("src/lib.rs", 112, 9)
//!     .new_line(112, "#![warn(missing_docs)]")
//!     .annotate_note(9, 12, "")?
//!     .finish()
//!     .log_to_target(&target)?;
//! # Ok(()) }
//! ```
//! This will output the following:
//! ```text
//! warning: missing documentation for an associated function
//!     --> src/lib.rs:1163:5
//!      |
//! 1163 |     pub fn log_to_target(self, target: &Target) {
//!      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//! note: the lint level is defined here
//!    --> src/lib.rs:112:9
//!     |
//! 112 | #![warn(missing_docs)]
//!     |         ^^^^^^^^^^^^
//! ```
//! Instead, we could "pack together" the entries:
//! ```
//! // Example available at `examples/doc_multi_entry_second.rs`.
//! // Use the feature `console` to emit a colored output.
//!
//! # fn main() -> prologue_logger::error::Result<()> {
//! # use prologue_logger::{Entry, MultiEntry, Target, Task};
//! // Target `example`
//! let target = Target::new("example");
//!
//! // Emit a warning for missing documentation.
//! let warn_missing_doc = Entry::new_warning("missing documentation for an associated function")
//!     .named_source("src/lib.rs", 1163, 5)
//!     .new_line(1163, "    pub fn log_to_target(self, target: &Target) {")
//!     .annotate_warn(5, 43, "")?
//!     .finish();
//! // Then, emit a note about the lint level.
//! let note_lint_level = Entry::new_note("the lint level is defined here")
//!     .named_source("src/lib.rs", 112, 9)
//!     .new_line(112, "#![warn(missing_docs)]")
//!     .annotate_note(9, 12, "")?
//!     .finish();
//! // Finally, create the `MultiEntry`, which contains...
//! MultiEntry::new()
//!     // ... the first warning entry and...
//!     .entry(warn_missing_doc)
//!     // ... the second note entry;
//!     .entry(note_lint_level)
//!     // then, log the `MultiEntry`.
//!     .log_to_target(&target);
//! # Ok(()) }
//! ```
//! Now the output will look like this:
//! ```text
//! warning: missing documentation for an associated function
//!     --> src/lib.rs:1163:5
//!      |
//! 1163 |     pub fn log_to_target(self, target: &Target) {
//!      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! note: the lint level is defined here
//!     --> src/lib.rs:112:9
//!      |
//! 112  | #![warn(missing_docs)]
//!      |         ^^^^^^^^^^^^
//! ```
//! Notice that the lines are now aligned and there is no gap between the entries.
//!
//! # Features
//!
//! ## [`console`](https://docs.rs/console/0.15.0/console/)
//!
//! As introduced before, the `console` feature will produce colored output.
//! Ideally, you should not use the `console` feature when outputting to a file;
//! on the other side, the `console` feature makes the output to `stderr` more readable.
//!
//! For instance, the output
//! ```text
//! warning: missing documentation for an associated function
//!     --> src/lib.rs:1163:5
//!      |
//! 1163 |     pub fn log_to_target(self, target: &Target) {
//!      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! note: the lint level is defined here
//!     --> src/lib.rs:112:9
//!      |
//! 112  | #![warn(missing_docs)]
//!      |         ^^^^^^^^^^^^
//! ```
//! from the previous example would have the line numbers and bars colored in bright cyan,
//! the `warning` label and respective `^^^^` annotation colored in bright yellow,
//! and the `note` label and respective `^^^^` annotation colored in bright green.
//!
//! ## [`indicatif`](https://docs.rs/indicatif/0.17.0-rc.10/indicatif/index.html)
//!
//! This crate also integrates with `indicatif`.
//! In particular, it is possible to insert progress bars into a target or a target list
//! so that the logged output is always above the progress bar.
#![cfg_attr(not(feature = "indicatif"), doc = "```ignore")]
#![cfg_attr(feature = "indicatif", doc = "```")]
//! # fn main() -> prologue_logger::error::Result<()> {
//! # use indicatif::{ProgressBar, ProgressStyle};
//! # use prologue_logger::{Entry, Task, TargetList};
//! // Create target `example`.
//! let target_list = TargetList::new();
//! let target = target_list.create_target("example")?;
//! // Create progress bar.
//! let pb = ProgressBar::new(4);
//! pb.set_style(ProgressStyle::default_bar());
//! // Add progress bar to target list.
//! target_list.add_progress_bar(pb.clone());
//!
//! // Do stuff
//! for i in 0..4 {
//!     // Log to target.
//!     Task::new("Doing", format!("stuff {}", i + 1))
//!         .log_to_target(&target)?;
//!     // Do some intensive stuff.
//!     std::thread::sleep(std::time::Duration::from_millis(800));
//!     // Increase progress after finishing.
//!     pb.inc(1);
//! }
//!
//! # Ok(()) }
//! ```

//#![warn(missing_docs)]

use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
#[cfg(feature = "log")]
use log::{LevelFilter, Metadata, Record};

pub mod error;
pub mod style;
mod internals;

use error::{Result, ErrorKind};
use internals::*;
use crate::style::{NoStyler, Styled, StyledLineStart, Styler, StylerTemplate};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum NoteKind {
    Help,
    Note
}
impl NoteKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            NoteKind::Help => "help",
            NoteKind::Note => "note"
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Note {
    kind: NoteKind,
    text: String,
}
impl Styled for Note {
    fn fmt_styled(&self, f: &mut Formatter<'_>, styler: &'static dyn Styler) -> std::fmt::Result {
        let len = f.width().unwrap_or(0);
        let mut lines = self.text.lines();
        // Write first line, if any.
        if let Some(line) = lines.next() {
            write!(f, "{: <len$}", StyledLineStart::new_bullet(styler), len = len)?;
            styler.fmt_str(f, StylerTemplate::Title, self.kind.as_str())?;
            writeln!(f, ": {}", line)?;
        }
        // Write other lines.
        for line in lines {
            write!(f, "{: <len$}", StyledLineStart::new_empty(styler), len = len)?;
            writeln!(f, "{: <len$} {}", "", line, len = self.kind.as_str().len() + 1)?;
        }
        // Finish.
        Ok(())
    }
}
impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = f.width().unwrap_or(0);
        let mut lines = self.text.lines();
        // Write first line, if any.
        if let Some(line) = lines.next() {
            let kind = self.kind.as_str();
            writeln!(f, "{: >len$} {}: {}", "=", kind, line, len = len + 2)?;
        }
        // Write other lines.
        for line in lines {
            writeln!(f, "{: >len$}       {}", " ", line, len = len + 2)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct AnnotationReference {
    position: usize,
    len: usize
}
impl PartialOrd for AnnotationReference {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}
impl Ord for AnnotationReference {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}
impl From<(usize, usize)> for AnnotationReference {
    fn from((position, len): (usize, usize)) -> Self {
        AnnotationReference { position, len }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Annotation {
    style: EntryKind,
    reference: AnnotationReference,
    text: String
}
impl PartialOrd for Annotation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.reference.partial_cmp(&other.reference)
    }
}
impl Ord for Annotation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.reference.cmp(&other.reference)
    }
}
impl Display for Annotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.style.style(&self.text))
    }
}
impl Annotation {
    fn advance(&self, offset: &mut usize, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{: >len$}", "", len = self.reference.position - *offset)?;
        *offset = self.reference.position;
        Ok(())
    }

    fn draw_underline(&self, offset: &mut usize, f: &mut Formatter) -> std::fmt::Result {
        self.advance(offset, f)?;
        write!(f, "{}", AnnotationUnderline(self.style, self.reference.len))?;
        *offset += self.reference.len;
        Ok(())
    }

    fn draw_text_arrow(&self, offset: &mut usize, f: &mut Formatter) -> std::fmt::Result {
        self.advance(offset, f)?;
        if self.text.len() > 0 {
            write!(f, "{: <len$}", self.style.style("|"), len = self.reference.len)?;
        } else {
            write!(f, "{: <len$}", "", len = self.reference.len)?;
        }
        *offset += self.reference.len;
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct SourceLine {
    line: usize,
    contents: String,
    annotations: Vec<Annotation>,

}
impl Display for SourceLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Get the offset of the line.
        let width = f.width().unwrap_or_else(|| format!("{}", self.line).len());
        writeln!(f, "{: <len$} {} {}", console::style(self.line).cyan().bright(), LineStart, self.contents, len = width)?;
        if self.annotations.len() > 0 {
            write!(f, "{: >len$} {}", "", LineStart, len = width)?;
            // Draw annotation lines.
            let mut offset = 0;
            for ann in self.annotations.iter() {
                ann.draw_underline(&mut offset, f)?;
            }
            // Draw annotation texts.
            let mut annotations: Vec<&Annotation> = self.annotations.iter().collect();
            // Draw first annotation.
            if let Some(ann) = annotations.pop() {
                write!(f, " {}", ann)?;
            }
            // Draw other annotations.
            while let Some(ann) = annotations.pop() {
                write!(f, "\n{: >len$} {}", "", LineStart, len = width)?;
                offset = 0;
                for prev_ann in annotations.iter() {
                    prev_ann.draw_text_arrow(&mut offset, f)?;
                }
                ann.draw_text_arrow(&mut offset, f)?;
                write!(f, "\n{: >len$} {}", "", LineStart, len = width)?;
                offset = 0;
                for prev_ann in annotations.iter() {
                    prev_ann.draw_text_arrow(&mut offset, f)?;
                }
                ann.advance(&mut offset, f)?;
                write!(f, "{}", ann)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl SourceLine {
    pub fn new<S: Into<String>>(line: usize, contents: S) -> SourceLine {
        let contents = contents.into();
        let annotations = Vec::new();
        SourceLine { line, contents, annotations,  }
    }
    pub fn annotate<R: Into<AnnotationReference>, S: Into<String>>(&mut self, style: EntryKind, reference: R, text: S) -> Result<()> {
        let reference = reference.into();
        let text = text.into();
        let annotation = Annotation { style, reference, text };
        for ann_ref in self.annotations.iter().map(|ann| &ann.reference) {
            if ann_ref.position + ann_ref.len <= annotation.reference.position {
                // The previous annotation ends before the start of the new annotation.
                // It is safe to skip.
                continue;
            }
            if annotation.reference.position + annotation.reference.len <= ann_ref.position {
                // The previous annotation starts after the end of the new annotation.
                // Since annotations are always sorted, it is safe to end here the loop.
                break;
            }
            // If we got so far, then some annotation is overlapping, return an error.
            return Err(ErrorKind::OverlappingAnnotation.into());
        }
        self.annotations.push(annotation);
        self.annotations.sort();
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Source {
    filename: Option<PathBuf>,
    line_number: usize,
    position: usize,
    lines: Vec<SourceLine>,
    notes: Vec<Note>
}
impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Get the offset of the line.
        let width = if let Some(width) = f.width() {
            width
        } else {
            let width = self.lines.iter()
                .map(|line| line.line)
                .max()
                .unwrap_or(self.line_number);
            format!("{}", width).len()
        };
        // Write "--> filename:row:position".
        if let Some(filename) = &self.filename {
            writeln!(f, "{: >len$}{} {}:{}:{}", "", Arrow, filename.display(), self.line_number, self.position, len = width)?;
        } else {
            writeln!(f, "{: >len$}{} <anonymous>:{}:{}", "", Arrow, self.line_number, self.position, len = width)?;
        }
        // Write an empty line.
        writeln!(f, "{: >len$} {}", "", LineStart, len = width)?;
        // Write all (annotated) source line.
        for line in self.lines.iter() {
            write!(f, "{:width$}", line, width = width)?;
        }
        // Write annotation texts.
        if let Some(line) = self.lines.last() {
            if line.annotations.len() == 0 {
                writeln!(f, "{: >len$} {}", "", LineStart, len = width)?;
            }
        } else {
            writeln!(f, "{: >len$} {}", "", LineStart, len = width)?;
        }

        // Write final notes.
        if self.notes.len() > 0 {
            writeln!(f, "{: >len$} {}", "", LineStart, len = width)?;
        }
        for note in self.notes.iter() {
            write!(f, "{:width$}", note, width = width)?;
        }
        Ok(())
    }
}
impl Source {
    pub fn new(line_number: usize, position: usize) -> Source {
        Source { filename: None, line_number, position, lines: Vec::new(), notes: Vec::new() }
    }

    pub fn set_filename<P: Into<PathBuf>>(&mut self, filename: P) {
        self.filename = Some(filename.into());
    }

    pub fn add_line(&mut self, line: SourceLine) {
        self.lines.push(line);
    }
}

/// Kind of the log line.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
enum EntryKind {
    /// Denotes an help.
    ///
    /// Usually used in additional lines for warnings or errors.
    Help,
    /// Denotes a note.
    ///
    /// Usually used in additional lines for warnings or errors.
    Note,
    /// Denotes a warning.
    Warning,
    /// Denotes an error.
    Error
}
impl Display for EntryKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryKind::Help => write!(f, "{}", console::style("help").cyan().bright()),
            EntryKind::Note => write!(f, "{}", console::style("note").green().bright()),
            EntryKind::Warning => write!(f, "{}", console::style("warning").yellow().bright()),
            EntryKind::Error => write!(f, "{}", console::style("error").red().bright())
        }
    }
}
impl EntryKind {
    fn style<D: Display>(&self, object: D) -> console::StyledObject<D> {
        match self {
            EntryKind::Help => console::style(object).cyan().bright(),
            EntryKind::Note => console::style(object).green().bright(),
            EntryKind::Warning => console::style(object).yellow().bright(),
            EntryKind::Error => console::style(object).red().bright()
        }
    }
}

/// Source builder for a log [`Entry`].
///
/// This `struct` has several methods which allow to customize the log entry
/// by adding source lines and annotations.
///
/// Constructed by the [`source`](Entry::source) or [`named_source`](Entry::named_source)
/// methods of [`Entry`].
#[derive(Clone, Debug)]
pub struct EntrySourceBuilder {
    entry: Entry,
    source: Source,
    source_line: Option<SourceLine>
}
impl EntrySourceBuilder {
    fn annotate<S: Into<String>>(mut self, kind: EntryKind, pos: usize, len: usize, text: S) -> Result<Self> {
        if let Some(ref mut line) = self.source_line {
            match line.annotate(kind, (pos, len), text) {
                Ok(_) => {},
                Err(mut err) => {
                    err.set_partial_configuration(self);
                    return Err(err);
                }
            }
        } else {
            return Err(ErrorKind::AnnotationOnEmptyLine.into_error_with_partial_configuration(self))
        }
        Ok(self)
    }

    /// Creates a new line in the entry, given the line number and the source code.
    ///
    /// # Example
    /// ```
    /// # let entry = prologue_logger::Entry::new_warning("");
    /// let source_line = "    file.read_to_string(&mut contents);";
    /// // Creates an entry for a piece of source code.
    /// // The entry refers to line 12, position 5.
    /// let entry_builder = entry.source(12, 5)
    ///     .new_line(12, source_line);
    /// ```
    pub fn new_line<S: Into<String>>(mut self, line_number: usize, source: S) -> Self {
        let line = SourceLine::new(line_number, source);
        if let Some(line) = self.source_line.replace(line) {
            self.source.lines.push(line);
        }
        self
    }

    /// Underlines a part of the source string and annotates the respective `text`.
    ///
    /// The `text` may be empty: in such case, the source will still be underlined
    /// but no additional effort is made to print the annotation text.
    ///
    /// The underline will be printed with `^` chars and, if the `console` feature is enabled,
    /// the color of the underline will be bright red.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::Entry;
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let source_line = "    file.read_to_string(&contents);";
    /// let pos = source_line.find("&contents").unwrap() + 1;
    /// let len = "&contents".len();
    ///
    /// let entry_builder = Entry::new_error("mismatched types")
    ///     .source(44, pos)
    ///     .new_line(44, source_line)
    /// // Annotates the error under "&contents", specifying a text.
    ///     .annotate_err(pos, len, "types differ in mutability")?;
    /// # Ok(()) }
    /// ```
    /// The above will result in the following:
    /// ```text
    ///   --> <anonymous>:44:25
    ///    |
    /// 44 |         file.read_to_string(&contents);
    ///    |                             ^^^^^^^^^ types differ in mutability
    /// ```
    ///
    /// # Errors
    ///
    /// If the given annotation overlaps with an already existing annotation for the same line,
    /// this will result in an error.
    /// ```
    /// # let entry = prologue_logger::Entry::new_error("some expression warning");
    /// let entry_builder = entry.source(44, 25)
    ///     .new_line(44, "    let result = 1 + 2 * 3;")
    /// // Annotate the product:                ^^^^^
    /// // This is the first annotation and it can be safely unwrapped.
    ///     .annotate_err(22, 5, "").unwrap();
    /// // Try to annotate the sum:         ++++*^^^^
    /// // This will result in an `OverlappingAnnotation` error.
    ///  assert!(entry_builder.clone().annotate_err(18, 5, "").is_err());
    /// // Try to annotate the operation:       ^^*^^
    /// // This will result in an `OverlappingAnnotation` error.
    ///  assert!(entry_builder.clone().annotate_err(24, 1, "").is_err());
    /// ```
    pub fn annotate_err<S: Into<String>>(self, pos: usize, len: usize, text: S) -> Result<Self> {
        self.annotate(EntryKind::Error, pos, len, text)
    }

    /// Underlines a part of the source string and annotates the respective `text`.
    ///
    /// The `text` may be empty: in such case, the source will still be underlined
    /// but no additional effort is made to print the annotation text.
    ///
    /// The underline will be printed with `^` chars and, if the `console` feature is enabled,
    /// the color of the underline will be bright yellow.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::Entry;
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let source_line = "use std::io::Read";
    /// let pos = source_line.find("std::io::Read").unwrap() + 1;
    /// let len = "std::io::Read".len();
    ///
    /// let entry_builder = Entry::new_warning("unused import: `std::io::Read`")
    ///     .source(6, pos)
    ///     .new_line(6, source_line)
    /// // Annotates the warning under "std::io::Read", without specifying any text.
    ///     .annotate_warn(pos, len, "")?;
    /// # Ok(()) }
    /// ```
    /// The above will result in the following:
    /// ```text
    ///  --> <anonymous>:6:5
    ///   |
    /// 6 | use std::io::Read;
    ///   |     ^^^^^^^^^^^^^
    /// ```
    ///
    /// # Errors
    ///
    /// If the given annotation overlaps with an already existing annotation for the same line,
    /// this will result in an error.
    /// ```
    /// # let entry = prologue_logger::Entry::new_warning("some expression warning");
    /// let entry_builder = entry.source(44, 25)
    ///     .new_line(44, "    let result = 1 + 2 * 3;")
    /// // Annotate the product:                ^^^^^
    /// // This is the first annotation and it can be safely unwrapped.
    ///     .annotate_warn(22, 5, "").unwrap();
    /// // Try to annotate the sum:         ++++*^^^^
    /// // This will result in an `OverlappingAnnotation` error.
    ///  assert!(entry_builder.clone().annotate_warn(18, 5, "").is_err());
    /// // Try to annotate the operation:       ^^*^^
    /// // This will result in an `OverlappingAnnotation` error.
    ///  assert!(entry_builder.clone().annotate_warn(24, 1, "").is_err());
    /// ```
    pub fn annotate_warn<S: Into<String>>(self, pos: usize, len: usize, text: S) -> Result<Self> {
        self.annotate(EntryKind::Warning, pos, len, text)
    }

    /// Underlines a part of the source string and annotates the respective `text`.
    ///
    /// The `text` may be empty: in such case, the source will still be underlined
    /// but no additional effort is made to print the annotation text.
    ///
    /// The underline will be printed with `^` chars and, if the `console` feature is enabled,
    /// the color of the underline will be bright green.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::Entry;
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let source_line = "#![warn(missing_docs)]";
    /// let pos = source_line.find("missing_docs").unwrap();
    /// let len = "missing_docs".len();
    ///
    /// let entry_builder = Entry::new_note("the lint level is defined here")
    ///     .source(1, pos)
    ///     .new_line(1, source_line)
    /// // Annotates the note under "missing_docs".
    ///     .annotate_note(pos, len, "")?;
    /// # Ok(())
    /// # }
    /// ```
    /// The above will result in the following:
    /// ```text
    ///  --> <anonymous>:1:9
    ///   |
    /// 1 | #![warn(missing_docs)]
    ///   |         ^^^^^^^^^^^^
    /// ```
    ///
    /// # Errors
    ///
    /// If the given annotation overlaps with an already existing annotation for the same line,
    /// this will result in an error.
    /// ```
    /// # let entry = prologue_logger::Entry::new_note("some expression warning");
    /// let entry_builder = entry.source(44, 25)
    ///     .new_line(44, "    let result = 1 + 2 * 3;")
    /// // Annotate the product:                ^^^^^
    /// // This is the first annotation and it can be safely unwrapped.
    ///     .annotate_note(22, 5, "").unwrap();
    /// // Try to annotate the sum:         ++++*^^^^
    /// // This will result in an `OverlappingAnnotation` error.
    ///  assert!(entry_builder.clone().annotate_note(18, 5, "").is_err());
    /// // Try to annotate the operation:       ^^*^^
    /// // This will result in an `OverlappingAnnotation` error.
    ///  assert!(entry_builder.clone().annotate_note(24, 1, "").is_err());
    /// ```
    pub fn annotate_note<S: Into<String>>(self, pos: usize, len: usize, text: S) -> Result<Self> {
        self.annotate(EntryKind::Note, pos, len, text)
    }

    /// Underlines a part of the source string and annotates the respective `text`.
    ///
    /// The `text` may be empty: in such case, the source will still be underlined
    /// but no additional effort is made to print the annotation text.
    ///
    /// The underline will be printed with `-` chars and, if the `console` feature is enabled,
    /// the color of the underline will be bright cyan.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::Entry;
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let source_line = "{};";
    /// let pos = 1;
    /// let len = 1;
    ///
    /// let entry_builder = Entry::new_error("expected `;`, found `{`")
    ///     .source(1, pos)
    ///     .new_line(6, source_line)
    /// // Annotates the help under "{".
    ///     .annotate_help(pos, len, "unexpected token")?;
    /// # Ok(())
    /// # }
    /// ```
    /// The above will result in the following:
    /// ```text
    ///  --> src\lib.rs:6:1
    ///   |
    /// 6 | {};
    ///   | - unexpected token
    /// ```
    ///
    /// # Errors
    ///
    /// If the given annotation overlaps with an already existing annotation for the same line,
    /// this will result in an error.
    /// ```
    /// # let entry = prologue_logger::Entry::new_help("some expression warning");
    /// let entry_builder = entry.source(44, 25)
    ///     .new_line(44, "    let result = 1 + 2 * 3;")
    /// // Annotate the product:                ^^^^^
    /// // This is the first annotation and it can be safely unwrapped.
    ///     .annotate_help(22, 5, "").unwrap();
    /// // Try to annotate the sum:         ++++*^^^^
    /// // This will result in an `OverlappingAnnotation` error.
    ///  assert!(entry_builder.clone().annotate_help(18, 5, "").is_err());
    /// // Try to annotate the operation:       ^^*^^
    /// // This will result in an `OverlappingAnnotation` error.
    ///  assert!(entry_builder.clone().annotate_help(24, 1, "").is_err());
    /// ```
    pub fn annotate_help<S: Into<String>>(self, pos: usize, len: usize, text: S) -> Result<Self> {
        self.annotate(EntryKind::Help, pos, len, text)
    }

    /// Adds a final note to the source.
    ///
    /// Multiple notes and helps can be added to a source.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::Entry;
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let entry_builder = Entry::new_error("mismatched types")
    ///     .source(44, 25)
    ///     .new_line(44, "    file.read_to_string(&contents);")
    ///     .annotate_err(25, 9, "types differ in mutability")?
    /// // Add a note to the error.
    ///     .note("expected mutable reference `&mut String`\n           found reference `&String`");
    /// # Ok(())
    /// # }
    /// ```
    /// The above will result in the following:
    /// ```text
    ///   --> src\lib.rs:44:29
    ///    |
    /// 44 |         file.read_to_string(&contents);
    ///    |                             ^^^^^^^^^ types differ in mutability
    ///    |
    ///    = note: expected mutable reference `&mut String`
    ///                       found reference `&String`
    /// ```
    pub fn note<S: Into<String>>(mut self, text: S) -> Self {
        self.source.notes.push(Note { kind: NoteKind::Note, text: text.into(),  });
        self
    }

    /// Adds a final help to the source.
    ///
    /// Multiple notes and helps can be added to a source.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::Entry;
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let entry_builder = Entry::new_error("mismatched types")
    ///     .source(3, 1)
    ///     .new_line(1, "async fn foo() {}")
    ///     .annotate_err(1, 5, "to use `async fn`, switch to Rust 2018 or later")?
    /// // Add an help to the error.
    ///     .help("set `edition = \"2021\"` in `Cargo.toml`");
    /// # Ok(())
    /// # }
    /// ```
    /// The above will result in the following:
    /// ```text
    ///  --> <anonymous>:3:1
    ///   |
    /// 3 | async fn foo() {}
    ///   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
    ///   |
    ///   = help: set `edition = "2021"` in `Cargo.toml`
    /// ```
    pub fn help<S: Into<String>>(mut self, text: S) -> Self {
        self.source.notes.push(Note { kind: NoteKind::Help, text: text.into(),  });
        self
    }

    /// Concludes the construction of the [`Entry`] and returns it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::Entry;
    /// let entry = Entry::new_error("mismatched types")
    ///     .source(44, 25)
    ///     // Use the builder pattern to construct the entry.
    ///     // ...
    ///     // Finish the construction and return the `Entry` again.
    ///     .finish();
    /// ```
    pub fn finish(self) -> Entry {
        let EntrySourceBuilder { mut entry, mut source, source_line } = self;
        if let Some(line) = source_line {
            source.add_line(line);
        };
        entry.source = Some(source);
        entry
    }

    /// Discards the construction and returns the original [`Entry`].
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Entry, EntrySourceBuilder};
    /// # fn build_entry(builder: EntrySourceBuilder) -> EntrySourceBuilder { builder }
    /// # fn validate_entry_in_some_way(builder: &EntrySourceBuilder) -> bool { true }
    /// let entry = Entry::new_error("mismatched types");
    /// let builder = entry.source(44, 25);
    /// // Use an external function to construct the entry.
    /// let builder = build_entry(builder);
    /// // If the entry is not valid, it is possible to discard it.
    /// let entry = if validate_entry_in_some_way(&builder) {
    ///     // The entry is valid, keep the changes.
    ///     builder.finish()
    /// } else {
    ///     // The entry is not valid, discard the changes.
    ///     builder.discard()
    /// };
    /// ```
    pub fn discard(self) -> Entry {
        let EntrySourceBuilder { entry, .. } = self;
        entry
    }
}

/// A log entry.
///
/// Contains all the information that needs to be displayed in the log and implements the
/// [`Display`](std::fmt::Display) trait to ease use in formatting macros line `format!`
/// or `print!`.
#[derive(Clone, Debug)]
pub struct Entry {
    kind: EntryKind,
    bright: bool,
    text: String,
    source: Option<Source>
}
impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let styled = console::style(&self.text);
        let styled = if self.bright { styled.white().bright() } else { styled };
        writeln!(f, "{}{} {}", self.kind, Colon, styled)?;
        if let Some(source) = &self.source {
            (source as &dyn Display).fmt(f)?;
        }
        if f.width().is_none() && self.source.is_some() {
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Entry {
    fn new<S: Into<String>>(kind: EntryKind, text: S) -> Entry {
        let text = text.into();
        Entry { kind, bright: false, text, source: None }
    }

    /// Creates a new error entry.
    ///
    /// # Example
    /// ```
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// # use prologue_logger::Entry;
    /// // Construct the error entry.
    /// let entry = Entry::new_error("something bad happened!");
    ///
    /// // Output the entry.
    /// print!("{}", entry);
    /// # Ok(())
    /// # }
    /// ```
    /// The above produces the following text to be printed.
    /// ```text
    /// error: something bad happened!
    /// ```
    /// If the feature `console` is enabled, the output will be colored as in `cargo`,
    /// i.e. the `error` string will be printed in bright red.
    ///
    /// For more complete examples, see the [crate help](crate)
    /// or the `examples` directory.
    pub fn new_error<S: Into<String>>(text: S) -> Entry {
        Entry::new(EntryKind::Error, text)
    }

    /// Creates a new warning entry.
    ///
    /// # Example
    /// ```
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// # use prologue_logger::Entry;
    /// // Construct the warning entry.
    /// let entry = Entry::new_warning("something bad may happen!");
    ///
    /// // Output the entry.
    /// print!("{}", entry);
    /// # Ok(())
    /// # }
    /// ```
    /// The above produces the following text to be printed.
    /// ```text
    /// warning: something bad may happen!
    /// ```
    /// If the feature `console` is enabled, the output will be colored as in `cargo`,
    /// i.e. the `warning` string will be printed in bright yellow.
    ///
    /// For more complete examples, see the [crate help](crate)
    /// or the `examples` directory.
    pub fn new_warning<S: Into<String>>(text: S) -> Entry {
        Entry::new(EntryKind::Warning, text)
    }

    /// Creates a new note entry.
    ///
    /// # Example
    /// ```
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// # use prologue_logger::Entry;
    /// // Construct the note entry.
    /// let entry = Entry::new_note("something happened!");
    ///
    /// // Output the entry.
    /// print!("{}", entry);
    /// # Ok(())
    /// # }
    /// ```
    /// The above produces the following text to be printed.
    /// ```text
    /// note: something happened!
    /// ```
    /// If the feature `console` is enabled, the output will be colored as in `cargo`,
    /// i.e. the `note` string will be printed in bright green.
    ///
    /// For more complete examples, see the [crate help](crate)
    /// or the `examples` directory.
    pub fn new_note<S: Into<String>>(text: S) -> Entry {
        Entry::new(EntryKind::Note, text)
    }

    /// Creates a new help entry.
    ///
    /// # Example
    /// ```
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// # use prologue_logger::Entry;
    /// // Construct the warning entry.
    /// let entry = Entry::new_help("this is an help message.");
    ///
    /// // Output the entry.
    /// print!("{}", entry);
    /// # Ok(())
    /// # }
    /// ```
    /// The above produces the following text to be printed.
    /// ```text
    /// help: this is an help message.
    /// ```
    /// If the feature `console` is enabled, the output will be colored as in `cargo`,
    /// i.e. the `help` string will be printed in bright cyan.
    ///
    /// For more complete examples, see the [crate help](crate)
    /// or the `examples` directory.
    pub fn new_help<S: Into<String>>(text: S) -> Entry {
        Entry::new(EntryKind::Help, text)
    }

    /// Creates an anonymous source code and allows to configure it.
    ///
    /// This function takes the `Entry` by value and outputs an [`EntrySourceBuilder`]
    /// `struct` which allows to configure it.
    ///
    /// # Example
    /// ```
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// # use prologue_logger::Entry;
    /// // Construct a new warning entry...
    /// let entry = Entry::new_warning("unused import: `std::io::Read`")
    ///     // ... for an anonymous in line 5, position 5;
    ///     .source(5, 5)
    ///     // then add a new source code line...
    ///     .new_line(5, "use std::io::Read;")
    ///     // ... and annotate an error on it in position 5 (i.e. start of `std::io::Read`);
    ///     // the annotation is long 13 characters (i.e. length of `std::io::Read`)
    ///     // and contains no further information;
    ///     .annotate_warn(5, 13, "")?
    ///     // finally, finish the construction and obtain the constructed entry.
    ///     .finish();
    /// // Output the log entry.
    /// print!("{}", entry);
    /// # Ok(())
    /// # }
    /// ```
    /// The above produces the following result.
    /// ```text
    /// warning: unused import: `std::io::Read`
    ///  --> <anonymous>:5:5
    ///   |
    /// 5 | use std::io::Read;
    ///   |     ^^^^^^^^^^^^^
    /// ```
    /// If the feature `console` is enabled, the output will be colored as in `cargo`,
    /// i.e. the `warning` string and the `^^^^^^^^^^^^^` annotation
    /// will be printed in bright yellow.
    ///
    /// For more complete examples, see the `examples` directory.
    pub fn source(mut self, line_number: usize, pos: usize) -> EntrySourceBuilder {
        let source = Source::new(line_number, pos);
        self.bright = true;
        EntrySourceBuilder { entry: self, source, source_line: None }
    }

    /// Creates a named source code and allows to configure it.
    ///
    /// This function takes the `Entry` by value and outputs an [`EntrySourceBuilder`]
    /// `struct` which allows to configure it.
    ///
    /// # Example
    /// ```
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// # use prologue_logger::Entry;
    /// // Construct a new warning entry...
    /// let entry = Entry::new_warning("unused import: `std::io::Read`")
    ///     // ... for a file named `src/lib.rs` in line 5, position 5;
    ///     .named_source("src/lib.rs", 5, 5)
    ///     // then add a new source code line...
    ///     .new_line(5, "use std::io::Read;")
    ///     // ... and annotate an error on it in position 5 (i.e. start of `std::io::Read`);
    ///     // the annotation is long 13 characters (i.e. length of `std::io::Read`)
    ///     // and contains no further information;
    ///     .annotate_warn(5, 13, "")?
    ///     // finally, finish the construction and obtain the constructed entry.
    ///     .finish();
    /// // Output the log entry.
    /// print!("{}", entry);
    /// # Ok(())
    /// # }
    /// ```
    /// The above produces the following result.
    /// ```text
    /// warning: unused import: `std::io::Read`
    ///  --> src\lib.rs:5:5
    ///   |
    /// 5 | use std::io::Read;
    ///   |     ^^^^^^^^^^^^^
    /// ```
    /// If the feature `console` is enabled, the output will be colored as in `cargo`,
    /// i.e. the `warning` string and the `^^^^^^^^^^^^^` annotation
    /// will be printed in bright yellow.
    ///
    /// For more complete examples, see the `examples` directory.
    pub fn named_source<P: Into<PathBuf>>(mut self, filename: P, line_number: usize, pos: usize) -> EntrySourceBuilder {
        let mut source = Source::new(line_number, pos);
        source.set_filename(filename);
        self.bright = true;
        EntrySourceBuilder { entry: self, source, source_line: None }
    }

    /// Logs the current `Entry` to the predefined `target`, consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Entry};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let target_list = PrologueLogger::init()?;
    /// // Create a new target.
    /// target_list.create_target("example")?;
    ///
    /// // Log an entry.
    /// Entry::new_note("the logger has been initialized")
    ///     // Log the entry.
    ///     .log("example");
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "log")]
    pub fn log<S: AsRef<str>>(self, target: S) {
        match self.kind {
            EntryKind::Error => log::error!(target: target.as_ref(), "{}", self),
            EntryKind::Warning => log::warn!(target: target.as_ref(), "{}", self),
            EntryKind::Note => log::info!(target: target.as_ref(), "{}", self),
            EntryKind::Help => log::debug!(target: target.as_ref(), "{}", self)
        }
    }

    /// Logs the current `Entry` to the given `target`, consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Entry};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let logger = PrologueLogger::new();
    /// // Create a new target.
    /// let target = logger.create_target("example")?;
    ///
    /// // Log an entry.
    /// Entry::new_note("the logger has been initialized")
    ///     // Log the entry.
    ///     .log_to_target(&target)?;
    /// # Ok(()) }
    /// ```
    pub fn log_to_target(self, target: &Target) -> Result<()> {
        target.log_entry(self)
    }

    /// Logs the current `Entry` to the predefined `target` inside a [`PrologueLogger`], consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Entry};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let logger = PrologueLogger::new();
    /// // Create a new target.
    /// logger.create_target("example")?;
    ///
    /// // Log an entry.
    /// Entry::new_note("the logger has been initialized")
    ///     // Log the entry.
    ///     .log_to_prologue_logger("example", &logger);
    /// # Ok(()) }
    /// ```
    pub fn log_to_prologue_logger<S: AsRef<str>>(self, target: S, logger: &PrologueLogger) -> Result<()> {
        let target = logger.target_list.find(target);
        if let Some(target) = target {
            target.log_entry(self)?;
        }
        Ok(())
    }
}

/// A log entry which is given by the composition of multiple instances of [`Entry`].
///
/// Contains all the information that needs to be displayed in the log and implements the
/// [`Display`](std::fmt::Display) trait to ease use in formatting macros line `format!`
/// or `print!`.
/// Moreover, when emitting the `MultiEntry`, the children entries will be consecutive
/// (i.e., not separated by empty lines) and aligned (i.e. all code lines start at the same
/// position).
#[derive(Clone, Debug)]
pub struct MultiEntry {
    entries: Vec<Entry>
}
impl Display for MultiEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.entries.iter()
            .map(|e| e.source.as_ref().map(|s| s.line_number).unwrap_or(1))
            .max().unwrap_or(1);
        let width = format!("{}", width).len();
        for entry in self.entries.iter() {
            write!(f, "{:width$}", entry, width = width)?;
        }
        writeln!(f)
    }
}
impl Default for MultiEntry {
    fn default() -> Self {
        MultiEntry {
            entries: Vec::new()
        }
    }
}
impl MultiEntry {
    /// Creates a new, empty `MultiEntry`.
    pub fn new() -> MultiEntry {
        Default::default()
    }

    /// Adds an entry to this `MultiEntry`.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Entry, MultiEntry};
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// // Create the first entry: a warning about the missing documentation.
    /// let warn_missing_doc = Entry::new_warning("missing documentation for a struct")
    ///     .source(123, 1)
    ///     .new_line(123, "pub struct MultiEntry {")
    ///     .annotate_warn(1, 21, "")?
    ///     .finish();
    /// // Create the second entry: a note about the lint level.
    /// let note_lint_level = Entry::new_note("the lint level is defined here")
    ///     .source(1, 9)
    ///     .new_line(1, "#![warn(missing_docs)]")
    ///     .annotate_note(9, 12, "")?
    ///     .finish();
    /// // Create the `MultiEntry`.
    /// let entry = MultiEntry::new()
    ///     // Add the first entry.
    ///     .entry(warn_missing_doc)
    ///     // Add the second entry.
    ///     .entry(note_lint_level);
    /// // Show the entry.
    /// print!("{}", entry);
    /// # Ok(()) }
    /// ```
    /// The output of the above code will be:
    /// ```text
    /// warning: missing documentation for a struct
    ///    --> <anonymous>:123:1
    ///     |
    /// 123 | pub struct MultiEntry {
    ///     | ^^^^^^^^^^^^^^^^^^^^^
    /// note: the lint level is defined here
    ///    --> <anonymous>:1:9
    ///     |
    /// 1   | #![warn(missing_docs)]
    ///     |         ^^^^^^^^^^^^
    /// ```
    /// Instead, by printing the entries singularly, the output would be:
    /// ```text
    /// warning: missing documentation for a struct
    ///    --> <anonymous>:123:1
    ///     |
    /// 123 | pub struct MultiEntry {
    ///     | ^^^^^^^^^^^^^^^^^^^^^
    ///
    /// note: the lint level is defined here
    ///  --> <anonymous>:1:9
    ///   |
    /// 1 | #![warn(missing_docs)]
    ///   |         ^^^^^^^^^^^^
    /// ```
    pub fn entry(mut self, entry: Entry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Logs the current `MultiEntry` to the predefined `target`, consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Entry, MultiEntry};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let target_list = PrologueLogger::init()?;
    /// // Create a new target.
    /// target_list.create_target("example")?;
    ///
    /// // Log a `MultiEntry`.
    /// let warning = Entry::new_warning("something happened");
    /// let note = Entry::new_note("some more explanation");
    /// MultiEntry::new()
    ///     .entry(warning)
    ///     .entry(note)
    ///     // Log the entry.
    ///     .log("example");
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "log")]
    pub fn log<S: AsRef<str>>(self, target: S) {
        let kind = self.entries.iter()
            .map(|e| e.kind)
            .max()
            .unwrap_or(EntryKind::Help);
        match kind {
            EntryKind::Error => log::error!(target: target.as_ref(), "{}", self),
            EntryKind::Warning => log::warn!(target: target.as_ref(), "{}", self),
            EntryKind::Note => log::info!(target: target.as_ref(), "{}", self),
            EntryKind::Help => log::debug!(target: target.as_ref(), "{}", self)
        }
    }

    /// Logs the current `MultiEntry` to the given `target`, consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Entry, MultiEntry};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let logger = PrologueLogger::new();
    /// // Create a new target.
    /// let target = logger.create_target("example")?;
    ///
    /// // Log a `MultiEntry`.
    /// let warning = Entry::new_warning("something happened");
    /// let note = Entry::new_note("some more explanation");
    /// MultiEntry::new()
    ///     .entry(warning)
    ///     .entry(note)
    ///     // Log the entry.
    ///     .log_to_target(&target);
    /// # Ok(()) }
    /// ```
    pub fn log_to_target(self, target: &Target) -> Result<()> {
        target.log_multi_entry(self)
    }

    /// Logs the current `MultiEntry` to the predefined `target` inside a [`PrologueLogger`],
    /// consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Entry, MultiEntry};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let logger = PrologueLogger::new();
    /// // Create a new target.
    /// logger.create_target("example")?;
    ///
    /// // Log a `MultiEntry`.
    /// let warning = Entry::new_warning("something happened");
    /// let note = Entry::new_note("some more explanation");
    /// MultiEntry::new()
    ///     .entry(warning)
    ///     .entry(note)
    ///     // Log the entry.
    ///     .log_to_prologue_logger("example", &logger);
    /// # Ok(()) }
    /// ```
    pub fn log_to_prologue_logger<S: AsRef<str>>(self, target: S, logger: &PrologueLogger) -> Result<()> {
        let target = logger.target_list.find(target);
        if let Some(target) = target {
            target.log_multi_entry(self)?;
        }
        Ok(())
    }
}

/// A log entry with no other information than a "verb" and some other text.
///
/// This kind of entry is useful when logging what the application is doing.
///
/// # Example
/// ```
/// # use prologue_logger::Task;
/// print!("{}", Task::new("Running", "example"));
/// print!("{}", Task::new("Doing", "task 1"));
/// print!("{}", Task::new("Doing", "task 2"));
/// ```
/// This will produce the following output.
/// ```text
///      Running example
///        Doing task 1
///        Doing task 2
/// ```
/// If the feature `console` is enabled, the output will be colored as in `cargo`,
/// i.e. the "verb" will be printed in bright green and the trailing text will be printed
/// in white.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Task(String, String);
impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{: >width$} {}", console::style(&self.0).green().bright(), self.1, width = f.width().unwrap_or(12))
    }
}
impl Task {
    /// Creates a new entry given a `task` and a `description`.
    pub fn new<S1: Into<String>, S2: Into<String>>(task: S1, description: S2) -> Task {
        Task(task.into(), description.into())
    }

    /// Logs the current `Task` to the given `target`, consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Task};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let logger = PrologueLogger::new();
    /// // Create a new target.
    /// let target = logger.create_target("example")?;
    ///
    /// // Log a `Task`.
    /// Task::new("Initializing", "example")
    ///     // Log the task.
    ///     .log_to_target(&target);
    /// # Ok(()) }
    /// ```
    pub fn log_to_target(self, target: &Target) -> Result<()> {
        target.log_inline_entry(self)
    }

    /// Logs the current `Task` to the predefined `target` inside a [`PrologueLogger`],
    /// consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Task};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let logger = PrologueLogger::new();
    /// // Create a new target.
    /// logger.create_target("example")?;
    ///
    /// // Log a `Task`.
    /// Task::new("Initializing", "example")
    ///     // Log the entry.
    ///     .log_to_prologue_logger("example", &logger);
    /// # Ok(()) }
    /// ```
    pub fn log_to_prologue_logger<S: AsRef<str>>(self, target: S, logger: &PrologueLogger) -> Result<()> {
        let target = logger.target_list.find(target);
        if let Some(target) = target {
            target.log_inline_entry(self)?;
        }
        Ok(())
    }

    /// Logs the current `Task` to the predefined `target`, consuming it.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, Task};
    /// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    /// // Initialize the logger.
    /// let target_list = PrologueLogger::init()?;
    /// // Create a new target.
    /// target_list.create_target("example")?;
    ///
    /// // Log a task.
    /// Task::new("Initializing", "example")
    ///     // Log the task.
    ///     .log("example");
    ///
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "log")]
    pub fn log<S: AsRef<str>>(self, target: S) {
        log::info!(target: target.as_ref(), "{}", self)
    }
}

/// Log target containing information about the number of logged warnings/errors.
#[derive(Clone, Debug)]
pub struct Target {
    name: Arc<Cow<'static, str>>,
    warnings: Arc<Mutex<usize>>,
    errors: Arc<Mutex<usize>>,
    styler: Arc<Box<dyn Styler>>,
    #[cfg(feature = "indicatif")]
    multi_progress: indicatif::MultiProgress,

}
impl Target {
    /// Creates a new target with the given `name`.
    pub fn new<S: Into<Cow<'static, str>>>(name: S) -> Target {
        let name = Arc::new(name.into());
        let warnings = Arc::new(Mutex::new(0));
        let errors = Arc::new(Mutex::new(0));
        let styler: Arc<Box<dyn Styler>> = Arc::new(Box::new(NoStyler));
        #[cfg(feature = "indicatif")]
        let multi_progress = indicatif::MultiProgress::new();
        Target { name, warnings, errors, styler, #[cfg(feature = "indicatif")] multi_progress }
    }

    /// Creates a new target with the given `name` and assigns an existing
    /// `MultiProgress` to it.
    #[cfg(feature = "indicatif")]
    pub fn with_multi_progress<S: Into<Cow<'static, str>>>(name: S, multi_progress: indicatif::MultiProgress) -> Target {
        let name = Arc::new(name.into());
        let warnings = Arc::new(Mutex::new(0));
        let errors = Arc::new(Mutex::new(0));
        let styler: Arc<Box<dyn Styler>> = Arc::new(Box::new(NoStyler));
        Target { name, warnings, errors, styler, multi_progress }
    }

    /// Obtains the name of this target.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::Target;
    /// let target = Target::new("my-target");
    /// assert_eq!(target.name(), "my-target");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Obtains the number of warnings received by this target.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Target, Entry};
    /// // Create the target.
    /// let target = Target::new("my-target");
    /// // Log the first warning.
    /// Entry::new_warning("some warning")
    ///     .log_to_target(&target);
    /// // Log the second warning.
    /// Entry::new_warning("some other warning")
    ///     .log_to_target(&target);
    /// // This is not a warning.
    /// Entry::new_error("something went wrong")
    ///     .log_to_target(&target);
    ///
    /// assert_eq!(target.warning_count(), 2);
    /// ```
    pub fn warning_count(&self) -> usize {
        *self.warnings.lock().unwrap()
    }

    /// Obtains the number of warnings received by this target.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Target, Entry};
    /// // Create the target.
    /// let target = Target::new("my-target");
    /// // Log the first error.
    /// Entry::new_error("something went wrong")
    ///     .log_to_target(&target);
    /// // Log the second error.
    /// Entry::new_error("something else went wrong")
    ///     .log_to_target(&target);
    /// // This is not an error.
    /// Entry::new_warning("some warning")
    ///     .log_to_target(&target);
    ///
    /// assert_eq!(target.error_count(), 2);
    /// ```
    pub fn error_count(&self) -> usize {
        *self.errors.lock().unwrap()
    }

    fn log_entry(&self, entry: Entry) -> Result<()> {
        match entry.kind {
            EntryKind::Error => { *self.errors.lock().unwrap() += 1; },
            EntryKind::Warning => { *self.warnings.lock().unwrap() += 1; },
            _ => {}
        }
        #[cfg(not(feature = "indicatif"))]
        eprint!("{}", entry);
        #[cfg(feature = "indicatif")]
        self.multi_progress.println(format!("{}", entry))?;
        Ok(())
    }

    fn log_multi_entry(&self, multi: MultiEntry) -> Result<()> {
        let kind = multi.entries.iter()
            .map(|e| e.kind)
            .max()
            .unwrap_or(EntryKind::Help);
        match kind {
            EntryKind::Error => { *self.errors.lock().unwrap() += 1; },
            EntryKind::Warning => { *self.warnings.lock().unwrap() += 1; },
            _ => {}
        }
        #[cfg(not(feature = "indicatif"))]
        eprint!("{}", multi);
        #[cfg(feature = "indicatif")]
        self.multi_progress.println(format!("{}", multi))?;
        Ok(())
    }

    fn log_inline_entry(&self, entry: Task) -> Result<()> {
        #[cfg(not(feature = "indicatif"))]
        eprint!("{}", entry);
        #[cfg(feature = "indicatif")]
            self.multi_progress.println(format!("{}", entry))?;
        Ok(())
    }

    /// Logs a generic log record, increasing the warning/error count accordingly.
    #[cfg(any(feature = "log"))]
    pub fn log_record(&self, record: &log::Record) -> Result<()> {
        match record.level() {
            log::Level::Error => { *self.errors.lock().unwrap() += 1; },
            log::Level::Warn => { *self.warnings.lock().unwrap() += 1; },
            _ => {}
        }
        #[cfg(not(feature = "indicatif"))]
        eprint!("{}", record.args());
        #[cfg(feature = "indicatif")]
        self.multi_progress.println(format!("{}", record.args()))?;
        Ok(())
    }

    /// Executes the given `callback` if the target received at least one warning.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Target, Entry};
    /// // Create the target.
    /// let target = Target::new("my-target");
    /// // Create a general-purpose target.
    /// let global_target = Target::new("");
    /// // Log some warning
    /// Entry::new_warning("some warning")
    ///     .log_to_target(&target);
    ///
    /// // Output the fact that warnings were detected.
    /// target.if_warnings(|count| {
    ///     Entry::new_warning(format!("`my-target` generated {} warning{}", count, if count > 1 { "s" } else { "" }))
    ///         .log_to_target(&global_target)
    /// });
    /// // The warning count is still 1 since the above closure outputted to the general-purpose target.
    /// assert_eq!(target.warning_count(), 1);
    /// // However, the general-purpose target now contains a warning.
    /// assert_eq!(global_target.warning_count(), 1);
    /// ```
    pub fn if_warnings<F: FnOnce(usize) -> Result<()>>(&self, callback: F) -> Result<()> {
        let warning_count = self.warning_count();
        if warning_count > 0 {
            callback(warning_count)
        } else {
            Ok(())
        }
    }

    /// Executes the given `callback` if the target received at least one error.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Target, Entry};
    /// // Create the target.
    /// let target = Target::new("my-target");
    /// // Create a general-purpose target.
    /// let global_target = Target::new("");
    /// // Log some error
    /// Entry::new_error("something went wrong")
    ///     .log_to_target(&target);
    ///
    /// // Output the fact that errors were detected.
    /// target.if_errors(|count| {
    ///     Entry::new_error(format!("Could not run `my-target` due to {} previous error{}", count, if count > 1 { "s" } else { "" }))
    ///         .log_to_target(&global_target)
    /// });
    /// // The error count is still 1 since the above closure outputted to the general-purpose target.
    /// assert_eq!(target.error_count(), 1);
    /// // However, the general-purpose target now contains an error.
    /// assert_eq!(global_target.error_count(), 1);
    /// ```
    pub fn if_errors<F: FnOnce(usize) -> Result<()>>(&self, callback: F) -> Result<()> {
        let error_count = self.error_count();
        if error_count > 0 {
            callback(error_count)
        } else {
            Ok(())
        }
    }
}

/// A list of log targets.
#[derive(Clone, Debug)]
pub struct TargetList {
    list: Arc<Mutex<Vec<Target>>>,
    #[cfg(feature = "indicatif")]
    multi_progress: indicatif::MultiProgress
}
impl Default for TargetList {
    fn default() -> Self {
        TargetList {
            list: Arc::new(Mutex::new(Vec::new())),
            #[cfg(feature = "indicatif")]
            multi_progress: indicatif::MultiProgress::new()
        }
    }
}
impl TargetList {
    /// Creates a new, empty list of targets.
    pub fn new() -> TargetList {
        Default::default()
    }

    /// Finds a target inside the list.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::TargetList;
    /// # fn errors_callback(_: usize) -> prologue_logger::error::Result<()> { Ok(()) }
    /// let target_list = TargetList::new();
    /// target_list.create_target("my-target");
    ///
    /// // Do some work.
    ///
    /// let target = target_list.find("my-target")
    ///     .expect("no target `my-target`");
    /// target.if_errors(errors_callback);
    /// ```
    pub fn find<S: AsRef<str>>(&self, name: S) -> Option<Target> {
        let name = name.as_ref();
        let target = self.list.lock().unwrap().iter()
            .find(|t| t.name.as_ref() == name)
            .map(|t| t.to_owned());
        target
    }

    /// Creates a new target inside the list and outputs it.
    /// 
    /// Unless the feature `indicatif` is enabled, this is equivalent to creating
    /// a new target using the [`Target::new`] constructor and then putting it into the list
    /// using the method [`add_target`](TargetList::add_target) of `TargetList`.
    /// 
    /// # Example
    /// ```
    /// # use prologue_logger::{Task, TargetList};
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let target_list = TargetList::new();
    /// let target = target_list.create_target("my-target")?;
    /// Task::new("Running", "`my-target` (dev + unoptimized)")
    ///     .log_to_target(&target);
    /// # Ok(()) }
    /// ```
    pub fn create_target<S: Into<Cow<'static, str>>>(&self, name: S) -> Result<Target> {
        #[cfg(not(feature = "indicatif"))]
        let target = Target::new(name);
        #[cfg(feature = "indicatif")]
        let target = Target::with_multi_progress(name, self.multi_progress.clone());
        self.add_target(target.clone())?;
        Ok(target)
    }

    /// Adds a previously created target inside the list.
    /// 
    /// # Example
    /// ```
    /// # use prologue_logger::{Target, TargetList};
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let target_list = TargetList::new();
    /// let target = Target::new("my-target");
    /// // Puts `my-target` into the list.
    /// target_list.add_target(target)?;
    /// # Ok(()) }
    /// ```
    pub fn add_target(&self, target: Target) -> Result<()> {
        if let Some(other_target) = self.find(target.name.as_ref()) {
            Err(ErrorKind::TargetAlreadyExists(other_target.name.to_string()).into())
        } else {
            self.list.lock().unwrap()
                .push(target);
            Ok(())
        }
    }

    /// Clears all the attached progress bars.
    #[cfg(feature = "indicatif")]
    pub fn clear_progress_bar(&self) -> Result<()> {
        self.multi_progress.clear()?;
        Ok(())
    }

    /// Adds a progress bar.
    ///
    /// See [`MultiProgress::add`](https://docs.rs/indicatif/latest/indicatif/struct.MultiProgress.html#method.add)
    /// for further information.
    #[cfg(feature = "indicatif")]
    pub fn add_progress_bar(&self, pb: indicatif::ProgressBar) {
        self.multi_progress.add(pb);
    }
}

/// The `prologue` logger `struct`.
///
/// It handles log entries and displays them to `stderr`.
#[derive(Debug)]
pub struct PrologueLogger {
    target_list: TargetList
}
impl PrologueLogger {
    /// Creates a new `PrologueLogger` with an empty target list.
    pub fn new() -> PrologueLogger {
        #[cfg(feature = "indicatif")]
        let multi_progress = indicatif::MultiProgress::new();
        PrologueLogger {
            target_list: TargetList { list: Arc::new(Mutex::new(Vec::new())), #[cfg(feature = "indicatif")] multi_progress }
        }
    }

    /// Initializes the `PrologueLogger` as the main logger with crate [`log`].
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Entry, PrologueLogger};
    /// fn main() -> prologue_logger::error::Result<()> {
    ///     // Initialize the logger.
    ///     let target_list = PrologueLogger::init()?;
    ///     // Create a new target.
    ///     target_list.create_target("my-target")?;
    ///     // Now it is possible to use the logger to log events.
    ///     Entry::new_note("the logger is now available")
    ///         // Simply use the `log` function inside `Entry`,
    ///         // `InlineEntry` or `MultiEntry` structs.
    ///         .log("my-target");
    ///
    ///     // Warnings and errors are counted as well...
    ///     let entry = Entry::new_warning("something needs your attention");
    ///     // ...even if logged through the `log` macros.
    ///     log::warn!(target: "my-target", "{}", entry);
    ///
    ///     let my_target = target_list.find("my-target")
    ///         .expect("no target `my-target`");
    ///     assert_eq!(my_target.warning_count(), 1);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "log")]
    pub fn init() -> Result<TargetList> {
        let logger = PrologueLogger::new();
        let target_list = logger.target_list();
        log::set_max_level(log::LevelFilter::Debug);
        log::set_boxed_logger(Box::new(logger))?;
        Ok(target_list)
    }

    /// Finds a target inside the target list.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, TargetList};
    /// # fn errors_callback(_: usize) -> prologue_logger::error::Result<()> { Ok(()) }
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let logger = PrologueLogger::new();
    /// logger.create_target("my-target")?;
    ///
    /// // Do some work.
    ///
    /// let target = logger.find_target("my-target")
    ///     .expect("no target `my-target`");
    /// target.if_errors(errors_callback);
    /// # Ok(()) }
    /// ```
    pub fn find_target<S: AsRef<str>>(&self, name: S) -> Option<Target> {
        self.target_list.find(name)
    }

    /// Returns the `TargetList` containing all the targets inside the logger.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{PrologueLogger, TargetList};
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let logger = PrologueLogger::new();
    /// // Create targets.
    /// logger.create_target("target-first")?;
    /// logger.create_target("target-second")?;
    /// // Get target list.
    /// let target_list = logger.target_list();
    /// // Check that the previously created targets are there.
    /// assert!(target_list.find("target-first").is_some());
    /// assert!(target_list.find("target-second").is_some());
    ///
    /// // Create another target.
    /// target_list.create_target("target-third")?;
    /// // Check that the above target is in the logger.
    /// assert!(logger.find_target("target-third").is_some());
    /// # Ok(()) }
    /// ```
    pub fn target_list(&self) -> TargetList {
        self.target_list.clone()
    }

    /// Creates a new target inside the target list and outputs it.
    ///
    /// Unless the feature `indicatif` is enabled, this is equivalent to creating
    /// a new target using the [`Target::new`] constructor and then putting it into the list
    /// using the method [`add_target`](PrologueLogger::add_target) of `PrologueLogger`.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Task, PrologueLogger};
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let logger = PrologueLogger::new();
    /// let target = logger.create_target("my-target")?;
    /// Task::new("Running", "`my-target` (dev + unoptimized)")
    ///     .log_to_target(&target);
    /// # Ok(()) }
    /// ```
    pub fn create_target<S: Into<Cow<'static, str>>>(&self, name: S) -> Result<Target> {
        self.target_list.create_target(name)
    }

    /// Adds a previously created target inside the target list.
    ///
    /// # Example
    /// ```
    /// # use prologue_logger::{Target, PrologueLogger};
    /// # fn main() -> prologue_logger::error::Result<()> {
    /// let logger = PrologueLogger::new();
    /// let target = Target::new("my-target");
    /// // Puts `my-target` into the list.
    /// logger.add_target(target)?;
    /// # Ok(()) }
    /// ```
    pub fn add_target(&self, target: Target) -> Result<()> {
        self.target_list.add_target(target)
    }
}
#[cfg(feature = "log")]
impl log::Log for PrologueLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() < LevelFilter::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(target) = self.find_target(record.target()) {
                target.log_record(record)
                    .expect("the logger encountered an `io` error and could not continue");
            }
        }
    }

    fn flush(&self) {}
}