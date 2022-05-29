use std::sync::{Arc, Mutex};
use indicatif::{ProgressBar, ProgressStyle};
use prologue_logger::{Entry, Task, MultiEntry, PrologueLogger, TargetList};

#[derive(Clone, Debug)]
pub struct Tasks {
    task_list: Arc<Mutex<Vec<&'static str>>>,
    current_tasks: Arc<Mutex<Vec<&'static str>>>,
    progress_bar: ProgressBar,
    target_list: TargetList
}
impl Tasks {
    pub fn from_some_data(target_list: TargetList) -> Tasks {
        let mut task_list = vec![
            "first",
            "second",
            "third",
            "fourth",
            "fifth",
            "sixth",
            "seventh",
            "eighth",
            "ninth",
            "tenth",
            "eleventh",
            "twelfth"
        ];
        task_list.reverse();
        let task_list = Arc::new(Mutex::new(task_list));
        let current_tasks = Arc::new(Mutex::new(Vec::new()));
        let progress_bar = ProgressBar::new(12);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{prefix:>12.cyan.bright} [{bar:27}] {pos:>2}/{len:2} {msg}").unwrap()
            .progress_chars("=> "));
        target_list.add_progress_bar(progress_bar.clone());
        progress_bar.set_prefix("Building");
        Tasks { task_list, current_tasks, progress_bar, target_list }
    }

    pub fn retrieve_next_task(&self) -> Option<&'static str> {
        let task = self.task_list.lock().unwrap().pop();
        if let Some(task) = task {
            let mut current_tasks = self.current_tasks.lock().unwrap();
            current_tasks.push(task);
            self.update_pb(&current_tasks);
            std::mem::drop(current_tasks);
        }
        task
    }

    pub fn do_the_task(&self, task_name: &'static str) {
        let compile_time = match task_name {
            "first" => 800,
            "second" => 350,
            "third" => 750,
            "fourth" => {
                Entry::new_warning("unused `Result` that must be used")
                    .named_source("sixth", 118, 5)
                    .new_line(118, "    target_list.create_target(\"\");")
                    .annotate_warn(4, 32, "").unwrap()
                    .note("this `Result` may be an `Err` variant, which should be handled")
                    .finish().log("fourth");
                225
            },
            "fifth" => 1150,
            "sixth" => {
                let multipart_1 = Entry::new_warning("missing documentation for a struct")
                    .named_source("sixth", 577, 1)
                    .new_line(577, "pub struct PrologueStderrLogger {")
                    .annotate_warn(1, 31, "").unwrap()
                    .finish();
                let multipart_2 = Entry::new_note("the lint level is defined here")
                    .named_source("sixth", 1, 9)
                    .new_line(1, "#![warn(missing_docs)]")
                    .annotate_note(9, 12, "").unwrap()
                    .finish();
                MultiEntry::new()
                    .entry(multipart_1)
                    .entry(multipart_2)
                    .log("sixth");
                Entry::new_warning("missing documentation for an associated function")
                    .named_source("sixth", 581, 5)
                    .new_line(581, "    pub fn new() -> PrologueStderrLogger {")
                    .annotate_warn(5, 36, "").unwrap()
                    .finish().log("sixth");
                Entry::new_warning("missing documentation for an associated function")
                    .named_source("sixth", 590, 5)
                    .new_line(590, "    pub fn init() -> PrologueStderrLogger {")
                    .annotate_warn(5, 37, "").unwrap()
                    .finish().log("sixth");
                780
            },
            "seventh" => 340,
            "eighth" => 550,
            "ninth" => 950,
            "tenth" => 250,
            "eleventh" => 400,
            "twelfth" => 750,
            _ => unreachable!()
        };
        std::thread::sleep(std::time::Duration::from_millis(compile_time));
    }

    pub fn complete_task(&self, task_name: &'static str) {
        let mut current_tasks = self.current_tasks.lock().unwrap();
        let task_index = current_tasks.iter().position(|t| *t == task_name);
        if let Some(index) = task_index {
            current_tasks.remove(index);
            self.progress_bar.inc(1);
        }
        self.update_pb(&current_tasks);
        std::mem::drop(current_tasks);
    }

    pub fn update_pb(&self, current_tasks: &std::sync::MutexGuard<Vec<&'static str>>) {
        let mut task_string = String::new();
        for task in current_tasks.iter() {
            let comma = if task_string.len() == 0 { "" } else { "," };
            task_string = format!("{}{}{}", task_string, comma, task);
        }
        self.progress_bar.set_message(task_string);
    }

    pub fn spawn_task_doer(&self) -> std::thread::JoinHandle<()> {
        let tasks = self.to_owned();
        std::thread::spawn(move || {
            loop {
                let next_task = tasks.retrieve_next_task();
                if let Some(task) = next_task {
                    tasks.target_list.create_target(task).unwrap();
                    Task::new("Compiling", task)
                        .log(task);
                    tasks.do_the_task(task);
                    let target = tasks.target_list.find(task).unwrap();
                    target.if_errors(|count| {
                        Entry::new_error(format!("Could not run `{}` due to {} previous error{}", task, count, if count > 1 { "s" } else { "" }))
                            .log(task);
                        Ok(())
                    }).unwrap();
                    target.if_warnings(|count| {
                        Entry::new_warning(format!("`{}` (example) generated {} warning{}", task, count, if count > 1 { "s" } else { "" }))
                            .log(task);
                        Ok(())
                    }).unwrap();
                    tasks.complete_task(task);
                } else {
                    break;
                }
            }
        })
    }
}

fn main() -> prologue_logger::error::Result<()> {
    // Initialize logger.
    let target_list = PrologueLogger::init().unwrap();
    // Main logger target.
    target_list.create_target("").unwrap();
    // Generate tasks.
    let tasks = Tasks::from_some_data(target_list.clone());

    // Write some notification that this is NOT `cargo` (although the output may seem very similar).
    Entry::new_warning("starting `log.rs` -- the following is NOT generated by `cargo`")
        .log("");

    // Spawn task doers.
    let thread_1 = tasks.spawn_task_doer();
    let thread_2 = tasks.spawn_task_doer();

    // Await for task doers.
    thread_1.join().unwrap();
    thread_2.join().unwrap();

    // Clear progress bar.
    target_list.clear_progress_bar()?;

    // Should add some check here.
    Task::new("Finished", "example `example/indicatif.rs` (this is NOT `cargo`!)")
        .log("");

    Ok(())
}