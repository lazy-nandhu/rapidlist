use crate::todo::Todo;
use clap::{arg, Arg, Command};

pub fn run_cmds() {
    let matches = Command::new("rapdlist")
        .name("Rapdlist")
        .version("0.9")
        .about("A simple todo list written in rust")
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("add")
                .about("Adds a task to the todo list")
                .short_flag('a')
                .arg(arg!([TASK])),
            Command::new("list")
                .about("Displays all todo from the list")
                .short_flag('l'),
            Command::new("delete")
                .about("Deletes a todo list of the provided ID")
                .short_flag('d')
                .arg(Arg::new("ID").value_parser(clap::value_parser!(usize))),
            Command::new("check")
                .about("Marks the task completed of the provided ID")
                .short_flag('c')
                .arg(Arg::new("ID").value_parser(clap::value_parser!(usize))),
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let task: &String = sub_matches.get_one::<String>("TASK").expect("Required");
            let todo = Todo::new(
                Todo::next_id(),
                String::from(task),
                false,
                Todo::time_now(),
                None,
            );
            Todo::write_todo(todo);
        }
        Some(("list", _sub_matches)) => {
            Todo::display();
        }
        Some(("delete", sub_matches)) => {
            let id: &usize = sub_matches
                .get_one::<usize>("ID")
                .expect("Failed to get ID");
            Todo::remove_todo(*id);
        }
        Some(("check", sub_matches)) => {
            let id: &usize = sub_matches
                .get_one::<usize>("ID")
                .expect("Failed to get ID");
            Todo::mark_completed(*id);
        }
        _ => unreachable!("EX"),
    }
}
