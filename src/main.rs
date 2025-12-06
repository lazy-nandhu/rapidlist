#[derive(Debug)]
struct Todo {
    id: u32,
    task: String,
    completed: bool,
    created_at: String,
    completed_at: Option<String>,
}

impl Todo {
    fn new(
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

    fn display(&self) {
        todo!();
    }
}

fn display_is_completed(todo: &Todo) {
    match &todo.completed_at {
        Some(completed_at) => println!("is_completed: {}", completed_at),
        None => println!("is_completed: -"),
    }
}

fn main() {
    let todo2 = Todo::new(
        32,
        String::from("Hello world"),
        false,
        String::from("yesterday"),
        None,
    );
}
