// whatodo add "Make that one function"
// whatodo checkout all | Prints all todos
// whatodo checkout done | Prints all todos marked done
// whatodo checkout todo | Prints all todos not marked done
// whatodo complete 1 | Marks first todo as complete, will use 1 indexed list
// whatodo delete 1 | Deletes first todo, will use 1 indexed list
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
    let mut itf = File::open(format!(
        "{}{}",
        env::current_dir().unwrap().to_str().unwrap(),
        "todos.txt"
    ))?;
    let mut buf = String::new();

    itf.read_to_string(&mut buf)?;

    Ok(buf)
}

fn main_loop(args: &[String]) {
    let todos_list = load_todos(read_todos().unwrap());
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = &env_args[1..]; // Get arguments and collect all necessary for operation of the program

    let mut itf = File::open("todos.txt").unwrap(); // Open the todo file for processing
    let mut buf = String::new();

    itf.read_to_string(&mut buf).unwrap(); // Get all of the todos from the file

    let todos_list: Vec<Todo> = load_todos(buf);

    if args[0] == "checkout" {
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

    // let mut stuff = env::current_dir().unwrap();
    // stuff.push("todos.txt");

    // println!("{:?}", stuff);
}
