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

struct Todo {
    complete: bool,
    contents: String,
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
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{}|{}",
            match self.complete {
                true => 1,
                false => 0,
            },
            self.contents
        )
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

fn read_todos() -> Result<String, Error> {
    // Called by load_todos
    // Open todo file for retrieving previous todos
    let itf = File::open(format!(
        "{}{}",
        env::current_dir().unwrap().to_str().unwrap(),
        "todos.txt"
    )); // I do not check the return here so that I can send an empty string for use during runtime, later a new file will be created

    match itf {
        Ok(mut f) => {
            let mut buf = String::new();

            f.read_to_string(&mut buf)?;

            Ok(buf)
        }
        Err(_) => Ok(String::from("")),
    }
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
        otf.write(todo.to_string().as_bytes()).unwrap();
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = &env_args[1..]; // Get arguments and collect all necessary for operation of the program

    if args.len() == 0 {
        println!("Display help");
    } else {
        if args[0] == "init" {
            // If user has not created a todo, create it
            init_new_list().unwrap();
        } else {
            let mut todos_list: Vec<Todo> = load_todos();

            if args[0] == "checkout" {
                checkout_list(args[1].as_str(), &todos_list);
            } else if args[0] == "add" {
                // This branch involves all functionality for adding todos
                todos_list.push(Todo::new(None, String::from(args[1].clone())));
            } else if args[0] == "complete" {
                // This branch involves all functionality with completing todos
                match args[1].parse::<usize>() {
                    Ok(num) => {
                        todos_list[num - 1].complete = true;
                    }
                    Err(_) => println!("Invalid index given"),
                }
            } else if args[0] == "remove" {
                todos_list = remove_from_list(args[1].as_str(), todos_list);
            }

            save_todos(todos_list);
        }
    }

    // let mut stuff = env::current_dir().unwrap();
    // stuff.push("todos.txt");

    // println!("{:?}", stuff);
}
