// whatodo add "Make that one function"
// whatodo checkout all | Prints all todos
// whatodo checkout done | Prints all todos marked done
// whatodo checkout todo | Prints all todos not marked done
// whatodo complete 1 | Marks first todo as complete, will use 1 indexed list
// whatodo remove 1 | Deletes first todo, will use 1 indexed list
// whatodo remove done | Deletes all todos that are marked as completed
// whatodo remove todos | Deletes all todos that are not completed
// whatodo remove all | Deletes all todos from the current list
// whatodo init | Creates new list in current directory

use std::{
    fs::File,
    io::{Error, Read, Write},
    {env, fmt},
};

#[derive(Debug)]
struct Todo {
    complete: bool,
    contents: String,
    sub_todos: Vec<Todo>,
}

impl Todo {
    fn new(complete: Option<bool>, contents: String) -> Self {
        // Takes an option to allow for loading from file
        Self {
            complete: match complete {
                Some(b) => b,
                None => false,
            },
            contents,
            sub_todos: Vec::new(),
        }
    }

    fn to_string(&self) -> String {
        // Generally used for serialization
        if self.sub_todos.len() == 0 {
            format!(
                "{}|{}",
                match self.complete {
                    true => 1,
                    false => 0,
                },
                self.contents
            )
        } else {
            let mut str_repr = vec![format!(
                "{}|{}",
                match self.complete {
                    true => 1,
                    false => 0,
                },
                self.contents
            )];

            for child in self.sub_todos.iter() {
                let child_string = child
                    .to_string()
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n-");
                str_repr.push(format!("-{child_string}"));
            }

            str_repr.join("\n")
        }
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        // The sub todos will be the same generally speaking
        self.contents == other.contents
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.complete {
            write!(f, "[X] - {}", self.contents)
        } else {
            write!(f, "[ ] - {}", self.contents)
        }
    }
}

fn load_todos() -> Vec<Todo> {
    let mut itf = File::open("todos.txt").unwrap(); // Open the todo file for processing
    let mut todo_string = String::new();

    itf.read_to_string(&mut todo_string).unwrap(); // Get all of the todos from the file

    // Loads todos read in from file
    todo_string
        .split('\n') // All todos will be separated by a newline character
        .filter(|e| !e.is_empty()) // Get all non-empty todos from todos.txt
        .map(|e| e.split('|')) // The separator from complete and contents is the pipe character
        .map(|e| {
            let e_strs: Vec<&str> = e.collect();
            Todo::new(Some(e_strs[0] == "1"), String::from(e_strs[1].trim()))
        })
        .collect()
}

fn init_new_list() -> Result<(), Error> {
    File::create("todos.txt")?;
    Ok(())
}

fn checkout_list(option: &str, todos_list: &Vec<Todo>) {
    if option == "all" {
        for todo in todos_list {
            println!("{}", todo);
        }
    } else if option == "done" {
        for todo in todos_list.iter().filter(|e| e.complete) {
            println!("{}", todo);
        }
    } else if option == "todo" {
        for todo in todos_list.iter().filter(|e| !e.complete) {
            println!("{}", todo);
        }
    }
}

fn remove_from_list(option: &str, mut todos_list: Vec<Todo>) -> Vec<Todo> {
    // This branch involves all functionality with removing todos
    match option.parse::<usize>() {
        Ok(num) => {
            todos_list.remove(num - 1);
            todos_list
        }
        Err(_) => {
            if option == "all" {
                todos_list.clear();
                todos_list
            } else if option == "done" {
                todos_list.into_iter().filter(|e| !e.complete).collect()
            } else if option == "todo" {
                todos_list.into_iter().filter(|e| e.complete).collect()
            } else {
                todos_list
            }
        }
    }
}

fn save_todos(todos_list: Vec<Todo>) {
    let mut otf = File::create("todos.txt").unwrap();

    for todo in todos_list {
        otf.write(format!("{}\n", todo.to_string()).as_bytes())
            .unwrap();
    }
}

fn help() {
    println!("usage: whatodo <command> [<args>]");
    println!();
    println!("\tCreate a new todo list in current working directory:");
    println!();
    println!("\t\twhatodo init");
    println!();
    println!("\tAdd item to todo list in current working directory:");
    println!();
    println!("\t\twhatodo add 'Description of todo item here'");
    println!();
    println!("\tComplete an item on todo list:");
    println!();
    println!("\t\twhatodo complete (number of the item you would like to complete)");
    println!();
    println!("\tRemove an item from the todo list:");
    println!();
    println!("\t\twhatodo remove (number of the item you would like to complete)");
    println!();
    println!("\tDisplay items in todo list:");
    println!();
    println!("\t\twhatodo checkout (all|done|todo)");
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = &env_args[1..]; // Get arguments and collect all necessary for operation of the program

    // Use if let statements to check the length of the arguments sent in and give them descriptive values
    if let [command, value] = args {
        let mut todos_list: Vec<Todo> = load_todos();

        if command == "checkout" {
            checkout_list(value.as_str(), &todos_list);
        } else if command == "add" {
            // This branch involves all functionality for adding todos
            let new_todo = Todo::new(None, String::from(value.clone()));
            if !todos_list.contains(&new_todo) {
                todos_list.push(new_todo);
            } else {
                println!("Todo is already in the list")
            }
            save_todos(todos_list);
        } else if command == "complete" {
            // This branch involves all functionality with completing todos
            match value.parse::<usize>() {
                Ok(num) => {
                    todos_list[num - 1].complete = true;
                }
                Err(_) => println!("Invalid index given"),
            }
            save_todos(todos_list);
        } else if command == "remove" {
            todos_list = remove_from_list(value.as_str(), todos_list);
            save_todos(todos_list);
        }
    } else if let [command] = args {
        if command == "init" {
            // If user has not created a todo, create it
            init_new_list().unwrap();
        } else if command == "help" {
            help();
        }
    } else {
        if args.len() > 2 {
            eprintln!("Too many arguments provided!");
        }

        help();
    }
}
