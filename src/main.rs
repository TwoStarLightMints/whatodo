// All indices used below will be 1 indexed, as not all userers are to be assumed to know of 0 indexing
// whatodo add "Make that one function"    | Add a top level todo with contents indicated in quotations
// whatodo add 1 "A sub todo"              | Add a subtodo to the first top level todo with contents indicated in quotations
// whatodo add 1 1 "A sub todo's sub todo" | Add a subtodo to the first top level todo's first subtodo with contents indicated in quotations
// whatodo checkout all                    | Prints all todos
// whatodo checkout done                   | Prints all todos marked done
// whatodo checkout todo                   | Prints all todos not marked done
// whatodo complete 1                      | Marks first todo as complete
// whatodo complete 1 1                    | Marks first todo's first subtodo as complete
// whatodo remove 1                        | Deletes first todo, will use 1 indexed list
// whatodo remove done                     | Deletes all todos that are marked as completed
// whatodo remove todos                    | Deletes all todos that are not completed
// whatodo remove all                      | Deletes all todos from the current list
// whatodo remove 1 1                      | Deletes the first subtodo of the first todo
// whatodo init                            | Creates new list in current directory

use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use whatodo::{
    todo::{from_todo_string, Todo},
    utils,
};

fn load_todos() -> Result<Vec<Todo>, std::io::Error> {
    let mut f = File::open("todo.todos")?;

    let mut todo_string = String::new();

    f.read_to_string(&mut todo_string)?; // Get all of the todos from the file

    let mut todos: Vec<Todo> = Vec::new();

    // Loads todos read in from file
    for str in todo_string.lines().into_iter().filter(|s| !s.is_empty()) {
        todos.push(from_todo_string(str.to_string()));
    }

    Ok(todos)
}

fn init_new_list() -> Result<(), std::io::Error> {
    File::create("todo.todos")?;
    Ok(())
}

// There can exist multiple sub todos that are the same, but no base level todos may be the same
fn add_to_list(mut todos_list: Vec<Todo>, args: Vec<String>) {
    let depth_list = utils::depth_iterator_from_args_to_item(args.iter().skip(1).peekable());

    match args.last() {
        Some(value) => {
            let new_todo = Todo::new(None, value.clone());

            let mut curr_root = &mut todos_list;

            for ind in depth_list {
                curr_root = match curr_root.get_mut(ind) {
                    Some(node) => &mut node.sub_todos,
                    None => {
                        eprintln!("Index out of bounds, could not add todo");
                        return;
                    }
                }
            }

            if !utils::search_all_todos_content(&curr_root, &value) {
                curr_root.push(new_todo);
            } else {
                println!("Todo is already in the list")
            }

            save_todos(todos_list);
        }
        None => help(),
    }
}

fn checkout_list(todos_list: &Vec<Todo>, option: &str) {
    if todos_list.len() == 0 {
        println!("There are no todos!");
        return;
    }

    match option {
        "all" => {
            for (ind, todo) in todos_list.iter().enumerate() {
                println!("{}. {}", ind + 1, todo.to_enumerated_string(None));
            }
        }
        "done" => {
            for todo in todos_list.iter().filter(|e| e.complete) {
                println!("{}", todo.to_string());
            }
        }
        "todo" => {
            for (ind, todo) in todos_list.iter().filter(|e| !e.complete).enumerate() {
                println!("{}. {}", ind + 1, todo.to_enumerated_string(None));
            }
        }
        _ => help(),
    }
}

fn complete_todo(mut todos_list: Vec<Todo>, args: Vec<String>) {
    let item_to_complete = utils::get_mut_from_num_depth(
        &mut todos_list,
        &utils::depth_iterator_from_args_to_item(args.iter().peekable()),
    );

    match item_to_complete {
        Some(todo) => todo.complete = true,
        None => {
            eprintln!("Index is out of bounds, could not complete todo");
            return;
        }
    }

    save_todos(todos_list);
}

fn remove_from_list(mut todos_list: Vec<Todo>, args: Vec<String>) {
    match args.first() {
        Some(first) => {
            match first.as_str() {
                "all" => save_todos(Vec::new()),
                "done" => save_todos(todos_list.into_iter().filter(|t| !t.complete).collect()),
                "todo" => save_todos(todos_list.into_iter().filter(|t| t.complete).collect()),
                // Check to see if it's a depth thing
                _ => {
                    let mut depth_list =
                        utils::depth_iterator_from_args_to_parent(args.iter().peekable())
                            .into_iter();

                    let mut curr_root = &mut todos_list;

                    while let Some(ind) = depth_list.next() {
                        match curr_root.get_mut(ind) {
                            Some(node) => curr_root = &mut node.sub_todos,
                            None => {
                                eprintln!("Index out of bounds, could not remove todo");
                                return;
                            }
                        }
                    }

                    let index_to_remove = args.last().unwrap().parse::<usize>().unwrap() - 1;

                    match curr_root.get(index_to_remove) {
                        Some(_) => curr_root.remove(index_to_remove),
                        None => {
                            eprintln!("Index out of bounds, could not remove todo");
                            return;
                        }
                    };

                    save_todos(todos_list);
                }
            }
        }
        None => help(),
    }
}

fn save_todos(todos_list: Vec<Todo>) {
    let mut otf = File::create("todo.todos").unwrap();

    for todo in todos_list {
        otf.write(format!("{}\n", todo.to_todos()).as_bytes())
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
    println!("\tAdd sub item to a todo list in current working directory:");
    println!();
    println!("\t\twhatodo add [indices of todos] 'Description of todo item here'");
    println!();
    println!("\tComplete an item on todo list:");
    println!();
    println!("\t\twhatodo complete (number of the item you would like to complete)");
    println!();
    println!("\tRemove an item from the todo list:");
    println!();
    println!("\t\twhatodo remove (number of the item you would like to complete)");
    println!();
    println!("\tRemove a sub item from the todo list:");
    println!();
    println!("\t\twhatodo remove [indices of todos] subtodo");
    println!();
    println!("\tDisplay items in todo list:");
    println!();
    println!("\t\twhatodo checkout (all|done|todo)");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(command) = args.get(0) {
        match command.as_str() {
            "init" => init_new_list().unwrap(),
            "add" => add_to_list(load_todos().unwrap(), args),
            "remove" => remove_from_list(load_todos().unwrap(), args[1..].to_owned()),
            "complete" => complete_todo(load_todos().unwrap(), args[1..].to_owned()),
            "checkout" => checkout_list(
                &load_todos().unwrap(),
                match args.get(1) {
                    Some(arg) => arg,
                    None => "all",
                },
            ),
            _ => help(),
        }
    } else {
        help();
    }
}
