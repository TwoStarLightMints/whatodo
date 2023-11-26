use std::fs::File;
use std::io::{Read, Write};

// Separate full todos, not Todos within sub_todos field

// New line == new todo
// Each field separated with |
// sub_todos array begin represented by [ and end by ]
// Separate todos in sub_todos array separated by ,

#[derive(Debug)]
struct Todo {
    complete: bool,
    contents: String,
    sub_todos: Vec<Todo>,
}

impl Todo {
    fn new() -> Self {
        Self {
            complete: false,
            contents: "".to_string(),
            sub_todos: Vec::new(),
        }
    }
}

fn to_string_todo(todo: &Todo) -> String {
    if todo.sub_todos.len() == 0 {
        format!(
            "{}|{}",
            match todo.complete {
                false => 0,
                true => 1,
            },
            todo.contents
        )
    } else {
        let mut to_return = format!(
            "{}|{}|[",
            match todo.complete {
                false => 0,
                true => 1,
            },
            todo.contents
        );

        for sub_todo in todo.sub_todos.iter() {
            to_return.push_str(format!("{},", to_string_todo(sub_todo)).as_str());
        }

        to_return.pop();

        to_return.push_str("]");

        to_return
    }
}

#[derive(Debug)]
enum TodoTokens {
    FieldSeparator, // |
    TodoArrBeg,     // [
    TodoArrEnd,     // ]
    TodoSeparator,  // %
    TodoValue(String),
}

fn tokenize_todo_string(todo_str: &String) -> Vec<TodoTokens> {
    println!("{todo_str}");
    let mut str_chars = todo_str.chars();

    let mut tokens: Vec<TodoTokens> = Vec::new();

    // Get the complete value
    tokens.push(TodoTokens::TodoValue(
        str_chars.by_ref().take_while(|e| *e != '|').collect(),
    ));

    tokens.push(TodoTokens::FieldSeparator);

    // Get the contents value
    tokens.push(TodoTokens::TodoValue(
        str_chars.by_ref().take_while(|e| *e != '|').collect(),
    ));

    tokens.push(TodoTokens::FieldSeparator);

    while let Some(c) = str_chars.by_ref().next() {
        match c {
            '|' => tokens.push(TodoTokens::FieldSeparator), // If bar is encountered, push FieldSeparator, go to next
            '%' => tokens.push(TodoTokens::TodoSeparator), // If percent sign is encountered, push TodoSeparator, go to next
            '[' => tokens.push(TodoTokens::TodoArrBeg), // If open bracket is encountered, push TodoArrBeg, go to next
            ']' => tokens.push(TodoTokens::TodoArrEnd), // If close bracket is encountered, push TodoArrEnd, go to next
            _ => {
                // Any other found text will be some type of value, therefore collect it into a new string
                let mut new_value = String::new();
                new_value.push(c);

                let rest_value = str_chars
                    .by_ref()
                    .take_while(|e| *e != '|')
                    .collect::<String>();

                new_value.push_str(&rest_value);

                tokens.push(TodoTokens::TodoValue(new_value));
                tokens.push(TodoTokens::FieldSeparator);
            }
        }
    }

    tokens
}

fn todo_from_tokens(tokens: Vec<TodoTokens>) -> Todo {}

fn from_todo_string(todo_str: String) -> Todo {
    let mut raw_tokens = tokenize_todo_string(&todo_str);
    let mut tokens = raw_tokens.iter();

    let mut root = Todo::new();

    if let Some(complete) = tokens.by_ref().next() {
        if let TodoTokens::TodoValue(val) = complete {
            root.complete = match val.as_str() {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            };
        }
    }

    tokens.by_ref().next(); // Skip field separator

    if let Some(contents) = tokens.by_ref().next() {
        if let TodoTokens::TodoValue(val) = contents {
            root.contents = val.clone();
        }
    }

    let child = todo_from_tokens(tokens.collect::<Vec<TodoTokens>>());

    println!("{root:?}");

    Todo {
        complete: false,
        contents: "".to_string(),
        sub_todos: Vec::new(),
    }
}
