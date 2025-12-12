use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::prelude::*;

use tabled::{
    derive::display,
    settings::{object::Columns, Alignment, Modify, Style, Width},
    Table, Tabled,
};

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Todo {
    #[tabled(rename = "#")]
    id: u32,
    #[tabled(rename = "Task")]
    task: String,
    #[tabled(rename = "Done ?")]
    completed: bool,
    #[tabled(rename = "Created At")]
    created_at: String,
    #[tabled(display("display::option", "-"))]
    #[tabled(rename = "Completed At")]
    completed_at: Option<String>,
}

impl Todo {
    pub fn time_now() -> String {
        let now = Local::now().format("%a %b %d %T").to_string();
        now
    }

    pub fn new(
        id: u32,
        task: String,
        completed: bool,
        created_at: String,
        completed_at: Option<String>,
    ) -> Todo {
        Todo {
            id,
            task,
            completed,
            created_at,
            completed_at,
        }
    }

    pub fn read_file() -> Vec<Todo> {
        let filename = "todos.json";
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let path = [root_dir, filename].join("/");
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        let todo_list: Vec<Todo> =
            serde_json::from_str(&contents).expect("Failed to convert string to Vec<Todo> ");
        todo_list
    }

    pub fn clear_n_write(data: String) {
        let filename = "todos.json";
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let path = [root_dir, filename].join("/");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .expect("Failed to open file");
        file.write_all(data.as_bytes())
            .expect("Failed to write to file");
    }

    pub fn write_todo(todo: Todo) {
        let mut todo_list: Vec<Todo> = Todo::read_file();
        todo_list.push(todo);
        let json_todo_list = serde_json::to_string_pretty(&todo_list).unwrap();
        Todo::clear_n_write(json_todo_list);
        println!("Task added!");
    }

    pub fn next_id() -> u32 {
        let todo_list: Vec<Todo> = Todo::read_file();
        let current_index = todo_list.len() as u32;
        current_index + 1
    }

    pub fn mark_completed(id: usize) {
        let mut todo_list: Vec<Todo> = Todo::read_file();
        let now = Todo::time_now();
        let id = id - 1;
        let index = id;
        let todo: &mut Todo = &mut todo_list[index];
        todo.completed = true;
        todo.completed_at = Some(now);
        let json_todo_list = serde_json::to_string_pretty(&todo_list)
            .expect("Failed to convert Vec<Todo> to json string");
        Todo::clear_n_write(json_todo_list);
        println!("Task marked completed!");
    }

    pub fn remove_todo(index: usize) {
        let id: u32 = index as u32;
        let index: u32 = id - 1;
        let mut todo_list: Vec<Todo> = Todo::read_file();
        if index as usize > todo_list.len() {
            println!("No todos found!");
            return;
        } else {
            todo_list.remove(index as usize);
            for n in 0..todo_list.len() {
                let index_usize = n + 1;
                todo_list[n].id = index_usize as u32;
            }
            let json_todo_list = serde_json::to_string_pretty(&todo_list).unwrap();
            Todo::clear_n_write(json_todo_list);
            println!("Task removed!");
        }
    }

    pub fn display() {
        let todo_list: Vec<Todo> = Todo::read_file();
        let mut table = Table::new(todo_list);

        table.with(Modify::new(Columns::new(1..=1)).with(Width::wrap(50).keep_words(true)));
        table.with(Style::sharp());
        table.modify(Columns::first(), Alignment::right());
        table.modify(Columns::new(3..=4), Alignment::center());

        println!("{}", table);
    }
}
