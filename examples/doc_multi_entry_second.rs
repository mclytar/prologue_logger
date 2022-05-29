use prologue_logger::{Entry, MultiEntry, Target};

fn main() -> prologue_logger::error::Result<()> {
    // Example available at `examples/doc_multi_entry_second.rs`.
    // Use the feature `console` to emit a colored output.

    // Target `example`
    let target = Target::new("example");

    // Emit a warning for missing documentation.
    let warn_missing_doc = Entry::new_warning("missing documentation for an associated function")
        .named_source("src/lib.rs", 1163, 5)
        .new_line(1163, "    pub fn log_to_target(self, target: &Target) {")
        .annotate_warn(5, 43, "")?
        .finish();
    // Then, emit a note about the lint level.
    let note_lint_level = Entry::new_note("the lint level is defined here")
        .named_source("src/lib.rs", 112, 9)
        .new_line(112, "#![warn(missing_docs)]")
        .annotate_note(9, 12, "")?
        .finish();
    // Finally, create the `MultiEntry`, which contains...
    MultiEntry::new()
        // ... the first warning entry and...
        .entry(warn_missing_doc)
        // ... the second note entry;
        .entry(note_lint_level)
        // then, log the `MultiEntry`.
        .log_to_target(&target)?;

    Ok(())
}