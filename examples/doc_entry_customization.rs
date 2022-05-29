use prologue_logger::{Entry, Target, Task};

fn main() -> prologue_logger::error::Result<()> {
    // Target receiving global messages.
    // Note: "" is just a name and has no particular meaning.
    let global_target = Target::new("");
    // Target receiving messages for `example`.
    let target = Target::new("example");

    // First, log the fact that we are compiling our example.
    Task::new("Compiling", "example v0.1.0")
        .log_to_target(&target)?;
    // Then, emit a warning for the mutable `x`.
    Entry::new_warning("variable does not need to be mutable")
        // The warning is for file `src/main.rs`, line 3, position 9.
        .named_source("src/main.rs", 3, 9)
        // Set the source line number (i.e., 3) and the respective source code.
        .new_line(3, "    let mut x = 42;")
        // Annotate the `mut ` string and add an help string.
        .annotate_help(9, 4, "help: remove this `mut`")?
        // Annotate the variable `x` (no help string needed).
        .annotate_warn(13, 1, "")?
        // Add a final note explaining why the user sees this warning.
        .note("`#[warn(unused_mut)]` on by default")
        // Finish the log line and output it.
        .finish()
        .log_to_target(&target)?;
    // Then, finish parsing the file `src/main.rs` and output a summary of the warnings.
    target.if_errors(|count|
        // This will not be displayed because no errors happened in `target`.
        Entry::new_error(format!("Could not run `example` due to {} previous error{}", count, if count > 1 { "s" } else { "" }))
            .log_to_target(&global_target))?;
    target.if_warnings(|count|
        // This will be displayed because at least one warning was generated in `target`.
        Entry::new_warning(format!("`example` (bin) generated {} warning{}", count, if count > 1 { "s" } else { "" }))
            // However, we don't want to add the "summary warning" to the warning count for `example`,
            // therefore we log it to the global target.
            .log_to_target(&global_target))?;
    // Finally, emit some entry telling that the compilation is finished.
    Task::new("Finished", "dev [unoptimized + debuginfo] target(s)")
        .log_to_target(&global_target)?;

    Ok(())
}