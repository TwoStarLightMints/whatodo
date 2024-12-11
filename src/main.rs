// All indices used below will be 1 indexed, as not all userers are to be assumed to know of 0 indexing
// whatodo add "Make that one function"   | Add a top level todo with contents indicated in quotations
// whatodo add 1 "A sub todo"             | Add a subtodo to the first top level todo with contents indicated in quotations
// whatodo add 11 "A sub todo's sub todo" | Add a subtodo to the first top level todo's first subtodo with contents indicated in quotations
// whatodo checkout all                   | Prints all todos
// whatodo checkout done                  | Prints all todos marked done
// whatodo checkout todo                  | Prints all todos not marked done
// whatodo complete 1                     | Marks first todo as complete
// whatodo complete 11                    | Marks first todo's first subtodo as complete
// whatodo remove 1                       | Deletes first todo, will use 1 indexed list
// whatodo remove done                    | Deletes all todos that are marked as completed
// whatodo remove todos                   | Deletes all todos that are not completed
// whatodo remove all                     | Deletes all todos from the current list
// whatodo remove 11 subtodo              | Deletes the first subtodo of the first todo
// whatodo init                           | Creates new list in current directory

use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use whatodo::todo::{from_todo_string, Todo};

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
fn add_to_list(mut todos_list: Vec<Todo>, value: String, num_depth: Option<&str>) {
    if todos_list
        .iter()
        .position(|t| t.contents == value)
        .is_none()
    {
        let new_todo = Todo::new(None, String::from(value.clone()));

        match num_depth {
            Some(depth) => {
                // The depth indicator 111 will push the sub todo to the first todo's first sub todo's first nested sub todo's sub todo list
                // root #1
                //     sub todo
                //         sub todo
                //             sub todo <- PUSHED HERE
                let mut depth_finder = depth
                    .chars()
                    .map(|c| usize::from_str_radix(String::from(c).as_str(), 10).unwrap() - 1)
                    .collect::<Vec<_>>()
                    .into_iter();

                let mut curr_root: &mut Vec<Todo> = &mut todos_list;

                while let Some(index) = depth_finder.next() {
                    if let Some(node) = curr_root.get_mut(index) {
                        curr_root = &mut node.sub_todos;
                    } else {
                        eprintln!("Index out of bounds, could not add subtodo");
                        return;
                    }
                }

                curr_root.push(new_todo);
            }
            None => todos_list.push(new_todo),
        }
    } else {
        println!("Todo is already in the list")
    }

    save_todos(todos_list);
}

fn checkout_list(option: &str, todos_list: &Vec<Todo>) {
    if todos_list.len() == 0 {
        println!("There are no todos!");
        return;
    }

    if option == "all" {
        for (ind, todo) in todos_list.iter().enumerate() {
            println!("{}. {}", ind + 1, todo.to_enumerated_string(None));
        }
    } else if option == "done" {
        for todo in todos_list.iter().filter(|e| e.complete) {
            println!("{}", todo.to_string());
        }
    } else if option == "todo" {
        for (ind, todo) in todos_list.iter().filter(|e| !e.complete).enumerate() {
            println!("{}. {}", ind + 1, todo.to_enumerated_string(None));
        }
    }
}

fn complete_todo(mut todos_list: Vec<Todo>, num_depth: &str) {
    let mut depth_finder = num_depth
        .chars()
        .map(|c| usize::from_str_radix(c.to_string().as_str(), 10).unwrap() - 1)
        .collect::<Vec<_>>()
        .into_iter();

    let final_index = depth_finder.next_back().unwrap();

    let mut curr_root = &mut todos_list;

    while let Some(ind) = depth_finder.next() {
        match curr_root.get_mut(ind) {
            Some(node) => curr_root = &mut node.sub_todos,
            None => {
                eprintln!("Index is out of bounds, could not complete todo");
                return;
            }
        }
    }

    match curr_root.get_mut(final_index) {
        Some(todo) => todo.complete = true,
        None => {
            eprintln!("Index is out of bounds, could not complete todo");
            return;
        }
    }

    save_todos(todos_list);
}

fn remove_from_list(
    option: &str,
    mut todos_list: Vec<Todo>,
    num_depth: Option<&str>,
) -> Result<Vec<Todo>, String> {
    // This branch involves all functionality with removing todos
    if let Some(depth) = num_depth {
        let mut depth_finder = depth
            .chars()
            .map(|c| usize::from_str_radix(String::from(c).as_str(), 10).unwrap() - 1)
            .collect::<Vec<_>>()
            .into_iter();

        let index = depth_finder.next().unwrap();
        // This is taken here, because we need to find the exact todo, unlike in the add todo function
        // we need to remove one specific sub todo from the list
        let final_index = depth_finder.next_back().unwrap();

        if index >= todos_list.len() {
            return Err(format!(
                "Index {} out of bounds, number of todos is {}",
                index,
                todos_list.len()
            ));
        }

        let mut root = &mut todos_list[index];

        while let Some(ind) = depth_finder.next() {
            if ind >= root.sub_todos.len() {
                return Err(format!(
                    "Index {} out of bounds, number of sub todos is {}",
                    ind,
                    root.sub_todos.len()
                ));
            }

            root = &mut root.sub_todos[ind];
        }

        root.sub_todos.remove(final_index);

        Ok(todos_list)
    } else {
        match option.parse::<usize>() {
            Ok(num) => {
                let ind_to_remove = num - 1;
                match todos_list.get(ind_to_remove) {
                    Some(_) => {
                        todos_list.remove(ind_to_remove);
                    }
                    None => println!("No more todos left!"),
                }
                Ok(todos_list)
            }
            Err(_) => {
                if option == "all" {
                    todos_list.clear();
                    Ok(todos_list)
                } else if option == "done" {
                    Ok(todos_list.into_iter().filter(|e| !e.complete).collect())
                } else if option == "todo" {
                    Ok(todos_list.into_iter().filter(|e| e.complete).collect())
                } else {
                    Ok(todos_list)
                }
            }
        }
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
    let env_args: Vec<String> = env::args().collect();
    let args = &env_args[1..]; // Get arguments and collect all necessary for operation of the program

    // Use if let statements to check the length of the arguments sent in and give them descriptive values
    if let [command, num_depth, value] = args {
        // This branch will handle adding a sub todo specifically
        let todos_list: Vec<Todo> = match load_todos() {
            Ok(list) => list,
            Err(message) => {
                eprintln!("{message}");
                std::process::exit(1);
            }
        };

        if command == "add" {
            add_to_list(todos_list, value.clone(), Some(num_depth));
        } else if command == "remove" {
            match remove_from_list(value.as_str(), todos_list, Some(num_depth)) {
                Err(e) => eprintln!("{}", e),
                Ok(list) => save_todos(list),
            }
        }
    } else if let [command, value] = args {
        let todos_list: Vec<Todo> = match load_todos() {
            Ok(list) => list,
            Err(message) => {
                eprintln!("{message}");
                std::process::exit(1);
            }
        };

        if command == "checkout" {
            checkout_list(value.as_str(), &todos_list);
        } else if command == "add" {
            // This branch involves all functionality for adding todos
            add_to_list(todos_list, value.clone(), None);
        } else if command == "complete" {
            // This branch involves all functionality with completing todos
            complete_todo(todos_list, value);
        } else if command == "remove" {
            match remove_from_list(value.as_str(), todos_list, None) {
                Err(e) => eprintln!("{e}"),
                Ok(list) => save_todos(list),
            }
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
