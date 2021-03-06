use prologue_logger::{Entry, Task, PrologueLogger};

fn main() -> prologue_logger::error::Result<()> {
    let logger = PrologueLogger::new();
    logger.create_target("simple").unwrap();
    logger.create_target("").unwrap();

    Entry::new_warning("starting `simple.rs` -- the following is NOT generated by `cargo`")
        .log_to_prologue_logger("simple", &logger)?;
    Task::new("Running", "example `simple.rs`")
        .log_to_prologue_logger("simple", &logger)?;
    Entry::new_warning("this is a warning line")
        .named_source("examples/simple.rs", 8, 36)
        .new_line(8, "    let entry = Entry::new_warning(\"this is a warning line\")")
        .annotate_help(9, 5, "this is the variable").unwrap()
        .annotate_help(17, 18, "this is the invoking function").unwrap()
        .annotate_note(36, 24, "this is the text").unwrap()
        .new_line(9, "    .bright()")
        .new_line(10, "    .source(source)")
        .new_line(11, "    .forward_to_stderr();")
        .annotate_warn(6, 17, "this function does not increase the warning count").unwrap()
        .note("this is not the actual source code")
        .help("to see the actual source code for this example,\nsee `examples/simple.rs`")
        .note("this output is generated by `prologue-logger` and NOT by `cargo`")
        .finish()
        .log_to_prologue_logger("simple", &logger)?;

    let target = logger.find_target("simple").unwrap();
    target.if_errors(|count| {
        Entry::new_warning(format!("Could not run `example/log.rs` due to {} previous error{}", count, if count > 1 { "s" } else { "" }))
            .log_to_prologue_logger("", &logger)
    })?;
    target.if_warnings(|count| {
        Entry::new_warning(format!("`example/simple.rs` (example) generated {} warning{}", count, if count > 1 { "s" } else { "" }))
            .log_to_prologue_logger("", &logger)
    })?;

    // No errors generated.
    assert_eq!(target.error_count(), 0);
    // Only two warnings logged (the third one was generated and displayed, but not logged).
    assert_eq!(target.warning_count(), 2);

    Ok(())
}