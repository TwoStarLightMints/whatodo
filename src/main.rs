// whatodo add "Make that one function"
// whatodo checkout all | Prints all todos
// whatodo checkout done | Prints all todos marked done
// whatodo checkout todo | Prints all todos not marked done
// whatodo complete 1 | Marks first todo as complete, will use 1 indexed list
// whatodo remove 1 | Deletes first todo, will use 1 indexed list
// whatodo remove done | Deletes all todos that are marked as completed
// whatodo remove todos | Deletes all todos that are not completed
// whatodo remove all | Deletes all todos from the current list
// whatodo new list | Creates new list in current directory

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

    fn complete(&mut self) {
        self.complete = true;
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

fn load_todos(todo_string: String) -> Vec<Todo> {
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

// fn main_loop(args: &[String]) {
//     let todos_list = load_todos(read_todos().unwrap());
// }

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = &env_args[1..]; // Get arguments and collect all necessary for operation of the program

    if args[0] == "init" {
        // If user has not created a todo, create it
        let mut _new_f = File::create("todos.txt").unwrap();

        _new_f.write("".as_bytes()).unwrap();
    } else {
        let mut itf = File::open("todos.txt").unwrap(); // Open the todo file for processing
        let mut buf = String::new();

        itf.read_to_string(&mut buf).unwrap(); // Get all of the todos from the file

        let mut todos_list: Vec<Todo> = load_todos(buf);

        if args[0] == "checkout" {
            // This branch involves all functionality for viewing todos
            if args[1] == "all" {
                for todo in &todos_list {
                    println!("{}", todo);
                }
            } else if args[1] == "done" {
                for todo in todos_list.iter().filter(|e| e.complete) {
                    println!("{}", todo);
                }
            } else if args[1] == "todo" {
                for todo in todos_list.iter().filter(|e| !e.complete) {
                    println!("{}", todo);
                }
            }
        } else if args[0] == "add" {
            // This branch involves all functionality for adding todos
            todos_list.push(Todo::new(None, String::from(args[1].clone())));
        } else if args[0] == "complete" {
            // This branch involves all functionality with completing todos
            match args[1].parse::<usize>() {
                Ok(num) => {
                    todos_list[num - 1].complete();
                }
                Err(_) => println!("Invalid index given"),
            }
        } else if args[0] == "remove" {
            // This branch involves all functionality with removing todos
            match args[1].parse::<usize>() {
                Ok(num) => {
                    todos_list.remove(num - 1);
                }
                Err(_) => {
                    if args[1] == "all" {
                        todos_list.clear();
                    } else if args[1] == "done" {
                        todos_list = todos_list.into_iter().filter(|e| !e.complete).collect();
                    } else if args[1] == "todo" {
                        todos_list = todos_list.into_iter().filter(|e| e.complete).collect();
                    }
                }
            }
        }

        let mut otf = File::create("todos.txt").unwrap();

        for todo in todos_list {
            otf.write(
                format!(
                    "{}|{}\n",
                    match todo.complete {
                        true => "1",
                        false => "0",
                    },
                    todo.contents
                )
                .as_bytes(),
            )
            .unwrap();
        }
    }

    // let mut stuff = env::current_dir().unwrap();
    // stuff.push("todos.txt");

    // println!("{:?}", stuff);
}
