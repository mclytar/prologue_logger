use prologue_logger::{Event, Target};
use prologue_logger::style::{ConsoleWriter, Highlight, StdWriter, Style, Styled, Title};

fn main() {
    let target = Target::with_styler("some_poetry.txt", ConsoleWriter);

    Event::new_task("Compiling", "`some_poetry.txt`")
        .log_to_target(&target).unwrap();
    Event::new_error(Title("wrong syntax near `writes`"))
        .file_reference("some_poetry.txt", 1, 3)
        .new_line()
        .new_source_code_line(1, "I writes good code")
        .new_line()
        .print(Highlight::with_style(Style::Error).offset(2).len(6))
        .new_line()
        .help("did you mean `write`?")
        .log_to_target(&target).unwrap();
}