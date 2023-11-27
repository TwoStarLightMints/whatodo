// Separate full todos, not Todos within sub_todos field

// New line == new todo
// Each field separated with |
// sub_todos array begin represented by [ and end by ]
// Separate todos in sub_todos array separated by ,

use std::fmt;

#[derive(Debug)]
pub struct Todo {
    pub complete: bool,
    pub contents: String,
    pub sub_todos: Vec<Todo>,
}

impl Todo {
    pub fn new(complete: Option<bool>, contents: String) -> Self {
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

    pub fn to_todos(&self) -> String {
        // Generally used for serialization
        if self.sub_todos.len() == 0 {
            format!(
                "{}|{}|",
                match self.complete {
                    true => 1,
                    false => 0,
                },
                self.contents
            )
        } else {
            let mut root = format!(
                "{}|{}|[",
                match self.complete {
                    true => 1,
                    false => 0,
                },
                self.contents
            );

            let mut children = Vec::new();

            for child in self.sub_todos.iter() {
                children.push(child.to_todos());
            }

            root.push_str(&children.join("%"));

            root.push(']');

            root
        }
    }

    pub fn to_string(&self) -> String {
        if self.sub_todos.len() == 0 {
            format!(
                "[{}] - {}",
                match self.complete {
                    false => ' ',
                    true => 'X',
                },
                self.contents
            )
        } else {
            let mut res = vec![format!(
                "[{}] - {}",
                match self.complete {
                    false => ' ',
                    true => 'X',
                },
                self.contents
            )];

            for child in self.sub_todos.iter() {
                let child_string = child.to_string();

                let complete = child_string
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n- ");

                res.push(format!("- {complete}"));
            }

            res.join("\n")
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

#[derive(Debug, Clone, PartialEq)]
enum TodoTokens {
    FieldSeparator, // |
    TodoArrBeg,     // [
    TodoArrEnd,     // ]
    TodoSeparator,  // %
    TodoValue(String),
}

fn tokenize_todo_string(todo_str: &String) -> Vec<TodoTokens> {
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

fn todo_from_tokens(tokens: Vec<TodoTokens>) -> Todo {
    let num_tokens = tokens.iter().count();
    let mut token_iter = tokens.iter();

    if num_tokens == 4 {
        // Todo with no nesting
        let mut todo = Todo::new(None, "".to_string());

        if let Some(TodoTokens::TodoValue(complete_field)) = token_iter.by_ref().next() {
            todo.complete = match complete_field.as_str() {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            };
        }

        token_iter.by_ref().next(); // This will be a field separator, so ignore

        if let Some(TodoTokens::TodoValue(contents_field)) = token_iter.next() {
            todo.contents = contents_field.clone();
        };

        todo
    } else {
        // Nested todos included
        let mut root = Todo::new(Some(false), "".to_string());

        if let Some(complete) = token_iter.by_ref().next() {
            if let TodoTokens::TodoValue(val) = complete {
                root.complete = match val.as_str() {
                    "0" => false,
                    "1" => true,
                    _ => unreachable!(),
                };
            }
        }

        token_iter.by_ref().next(); // Skip field separator

        if let Some(contents) = token_iter.by_ref().next() {
            if let TodoTokens::TodoValue(val) = contents {
                root.contents = val.clone();
            }
        }

        // Here, there is at least 1 TodoArrBeg and TodoArrEnd, there might be more, i.e. nested todos within the already nested todos, the
        // children variable here contains all remaing tokens after having parsed the first few above
        token_iter.next(); // Skip FieldSeparator
        token_iter.next(); // Skip TodoArrBeg
        token_iter.next_back(); // Skip TodoArrEnd

        let sub_todos = token_iter.map(|e| e.clone()).collect::<Vec<TodoTokens>>();

        let children = sub_todos.split(|e| *e == TodoTokens::TodoSeparator);

        for child in children {
            root.sub_todos.push(todo_from_tokens(child.to_vec()));
        }

        root
    }
}

// Format of Todo with no sub_todos: TodoValue, FieldSeparator, TodoValue, FieldSeparator
// Format of Todo with sub_todos: TodoValue, FieldSeparator, TodoValue, FieldSeparator, TodoArrBeg, ..., TodoArrEnd
fn from_todo_string(todo_str: String) -> Todo {
    let raw_tokens = tokenize_todo_string(&todo_str);

    todo_from_tokens(raw_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_todo_to_string_no_sub_todos() {
        let example = Todo::new(Some(false), "Something".to_string());
        assert_eq!("[ ] - Something", example.to_string());
    }

    #[test]
    fn from_todo_to_string_w_sub_todos() {
        let mut example = Todo::new(Some(false), "Something".to_string());

        example
            .sub_todos
            .push(Todo::new(Some(true), "This is a test".to_string()));

        assert_eq!(
            "[ ] - Something\n- [X] - This is a test",
            example.to_string()
        );
    }

    #[test]
    fn from_todo_to_string_w_nested_sub_todos() {
        let mut example = Todo::new(Some(false), "Something".to_string());

        example
            .sub_todos
            .push(Todo::new(Some(true), "This is a test".to_string()));

        example.sub_todos[0]
            .sub_todos
            .push(Todo::new(Some(false), "This is a nested test".to_string()));

        assert_eq!(
            "[ ] - Something\n- [X] - This is a test\n- - [ ] - This is a nested test",
            example.to_string()
        );
    }

    #[test]
    fn from_todo_to_todos_no_sub_todos() {
        let example = Todo::new(Some(false), "Something".to_string());
        assert_eq!("0|Something|", example.to_todos());
    }

    #[test]
    fn from_todo_to_todos_w_sub_todos() {
        let mut example = Todo::new(Some(false), "Something".to_string());

        example
            .sub_todos
            .push(Todo::new(Some(true), "This is a test".to_string()));

        example
            .sub_todos
            .push(Todo::new(Some(true), "This is a test".to_string()));

        example
            .sub_todos
            .push(Todo::new(Some(true), "This is a test".to_string()));

        assert_eq!(
            "0|Something|[1|This is a test|%1|This is a test|%1|This is a test|]",
            example.to_todos()
        );
    }

    #[test]
    fn from_todo_string_no_sub_todos() {
        assert_eq!(
            Todo {
                complete: false,
                contents: "Empty".to_string(),
                sub_todos: Vec::new()
            },
            from_todo_string("0|Empty|".to_string())
        );
    }

    #[test]
    fn from_todo_string_w_sub_todos() {
        assert_eq!(
            Todo {
                complete: false,
                contents: "One sub".to_string(),
                sub_todos: vec![Todo {
                    complete: true,
                    contents: "This is a sub_todo".to_string(),
                    sub_todos: Vec::new()
                }]
            },
            from_todo_string("0|One sub|[1|This is a sub_todo|]".to_string())
        );
    }

    #[test]
    fn from_todo_string_w_doubly_nested_sub_todos() {
        assert_eq!(
            Todo {
                complete: false,
                contents: "One sub".to_string(),
                sub_todos: vec![Todo {
                    complete: true,
                    contents: "This is a sub_todo".to_string(),
                    sub_todos: Vec::new()
                }]
            },
            from_todo_string(
                "0|One sub|[1|This is a sub_todo|[1|This is an even further nested todo|]]"
                    .to_string()
            )
        );
    }

    #[test]
    fn from_todo_string_w_multiple_sub_todos() {
        assert_eq!(
            Todo {
                complete: false,
                contents: "One sub".to_string(),
                sub_todos: vec![Todo {
                    complete: true,
                    contents: "This is a sub_todo".to_string(),
                    sub_todos: Vec::new()
                }]
            },
            from_todo_string(
                "0|One sub|[1|This is a sub_todo|%1|This is an even further nested todo|]"
                    .to_string()
            )
        );
    }
}
