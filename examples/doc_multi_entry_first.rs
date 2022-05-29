use prologue_logger::{Entry, Target};

fn main() -> prologue_logger::error::Result<()> {
    // Example available at `examples/doc_multi_entry_first.rs`.
    // Use the feature `console` to emit a colored output.

    // Target `example`
    let target = Target::new("example");

    // Emit a warning for missing documentation.
    Entry::new_warning("missing documentation for an associated function")
        .named_source("src/lib.rs", 1163, 5)
        .new_line(1163, "    pub fn log_to_target(self, target: &Target) {")
        .annotate_warn(5, 43, "")?
        .finish()
        .log_to_target(&target)?;
    // Then, emit a note about the lint level.
    Entry::new_note("the lint level is defined here")
        .named_source("src/lib.rs", 112, 9)
        .new_line(112, "#![warn(missing_docs)]")
        .annotate_note(9, 12, "")?
        .finish()
        .log_to_target(&target)?;

    Ok(())
}