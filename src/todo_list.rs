use std::{cell::Cell, fmt::Display};
use uuid::Uuid;

mod utils {
    use std::fmt::Display;
    use std::io::{self, Write};
    use std::process;

    pub fn read_stdin(msg: &str) -> io::Result<String> {
        let mut buffer = String::new();

        print!("{}: ", msg);

        io::stdout().flush()?;

        io::stdin().read_line(&mut buffer)?;

        Ok(buffer)
    }

    pub fn exit_err<T: Display>(msg: T, code: i32) -> ! {
        let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
        process::exit(code)
    }
}

enum Choice {
    Add,
    List,
    Complete,
    Exit,
}

struct TodoItem {
    id: String,
    description: String,
    is_completed: Cell<bool>,
}

impl TodoItem {
    pub fn new(description: String) -> Self {
        TodoItem {
            id: Uuid::new_v4().to_string(),
            description,
            is_completed: Cell::new(false),
        }
    }

    pub fn complete(&self) {
        self.is_completed.set(true)
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id - {}\nDescription - {}\nCompleted? - {}",
            self.id,
            self.description,
            self.is_completed.get()
        )
    }
}

pub fn main() {
    println!("To-do list");

    let mut todo_items: Vec<TodoItem> = Vec::new();

    loop {
        println!(
            "\nAdd - {}\nList - {}\nComplete - {}\nExit - {}",
            Choice::Add as u32,
            Choice::List as u32,
            Choice::Complete as u32,
            Choice::Exit as u32
        );

        let raw_choice = utils::read_stdin("\nInput")
            .unwrap_or_else(|e| utils::exit_err(&e, e.raw_os_error().unwrap_or(-1)))
            .trim()
            .parse::<u32>()
            .unwrap_or_else(|e| utils::exit_err(&e, 2));

        let choice = match raw_choice {
            0 => Choice::Add,
            1 => Choice::List,
            2 => Choice::Complete,
            3 => Choice::Exit,
            _ => utils::exit_err("Invalid choice", 2),
        };

        match choice {
            Choice::Exit => {
                println!("Bye!");
                break;
            }
            Choice::Add => {
                let description = utils::read_stdin("\nAdd to the todo item a description")
                    .unwrap_or_else(|e| utils::exit_err(&e, e.raw_os_error().unwrap_or(-1)))
                    .trim()
                    .parse::<String>()
                    .unwrap_or_else(|e| utils::exit_err(&e, 2));

                let todo_item = TodoItem::new(description);

                todo_items.push(todo_item);
            }
            Choice::List => {
                if todo_items.is_empty() {
                    println!("Is empty");
                    continue;
                }

                todo_items.iter().for_each(|item| println!("{}\n", item))
            }
            Choice::Complete => {
                let todo_item_id = utils::read_stdin("\nEnter the todo item id to complete")
                    .unwrap_or_else(|e| utils::exit_err(&e, e.raw_os_error().unwrap_or(-1)))
                    .trim()
                    .parse::<String>()
                    .unwrap_or_else(|e| utils::exit_err(&e, 2));

                let some_todo_item = todo_items.iter().find(|&item| item.id == todo_item_id);

                match some_todo_item {
                    None => println!("Item not found"),
                    Some(todo_item) => todo_item.complete(),
                }
            }
        }
    }
}
