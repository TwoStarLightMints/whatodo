// whatodo add "Make that one function"
// whatodo checkout all | Prints all todos
// whatodo checkout done | Prints all todos marked done
// whatodo checkout todo | Prints all todos not marked done
// whatodo complete 1 | Marks first todo as complete, will use 1 indexed list
// whatodo delete 1 | Deletes first todo, will use 1 indexed list
// whatodo new list | Creates new list in current directory

use std::{
    fs,
    io::Read,
    {env, fmt},
};

struct Todo {
    complete: bool,
    contents: String,
}

impl Todo {
    fn new(contents: String) -> Self {
        Self {
            complete: false,
            contents,
        }
    }

    fn load(complete: bool, contents: String) -> Self {
        Self { complete, contents }
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_clipped = &args[1..];

    let mut tf = fs::File::open("todos.txt").unwrap();
    let mut buf = String::new();

    tf.read_to_string(&mut buf);

    let todos_list: Vec<&str> = buf
        .split('\n')
        .filter(|e| !e.is_empty())
        .map(|e| e.trim())
        .collect();

    let final_todos: Vec<Vec<&str>> = todos_list
        .iter()
        .map(|e| e.split(' ').collect::<Vec<&str>>())
        .collect();

    let mutated_todos: Vec<Todo> = final_todos
        .iter()
        .map(|e| Todo::load(e[0] == "1", String::from(e[1])))
        .collect();

    if args_clipped[0] == "checkout" {
        if args_clipped[1] == "all" {
            for todo in mutated_todos {
                println!("{}", todo);
            }
        } else if args_clipped[1] == "done" {
            for todo in mutated_todos.iter().filter(|e| e.complete) {
                println!("{}", todo);
            }
        } else if args_clipped[1] == "todo" {
            for todo in mutated_todos.iter().filter(|e| !e.complete) {
                println!("{}", todo);
            }
        }
    }

    // let mut stuff = env::current_dir().unwrap();
    // stuff.push("todos.txt");

    // println!("{:?}", stuff);
}
