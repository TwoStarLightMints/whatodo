use crate::todo::Todo;
use std::iter::Peekable;

pub fn get_mut_from_num_depth<'a>(
    todos_list: &'a mut Vec<Todo>,
    num_depth: &str,
) -> Option<&'a mut Todo> {
    let mut depth_finder = num_depth
        .chars()
        .map(|c| usize::from_str_radix(c.to_string().as_str(), 10).unwrap() - 1)
        .collect::<Vec<_>>()
        .into_iter();

    let final_index = depth_finder.next_back().unwrap();

    let mut curr_root = todos_list;

    while let Some(ind) = depth_finder.next() {
        match curr_root.get_mut(ind) {
            Some(node) => curr_root = &mut node.sub_todos,
            None => {
                eprintln!("Index is out of bounds, could not complete todo");
                return None;
            }
        }
    }

    curr_root.get_mut(final_index)
}

pub fn get_depth_iterator_item<'a, I: Iterator<Item = &'a String> + std::fmt::Debug>(
    mut num_depth: Peekable<I>,
) -> Vec<usize> {
    //! Returns a vector which will contain the path to the individual item.
    let mut indices: Vec<usize> = Vec::new();

    while let Some(ind) = num_depth.next_if(|i| i.parse::<usize>().is_ok()) {
        indices.push(ind.parse::<usize>().unwrap() - 1);
    }

    indices
}

pub fn get_depth_iterator_list<'a, I: Iterator<Item = &'a str>>(
    num_depth: &'a mut Peekable<I>,
) -> Vec<usize> {
    //! Returns a vector which will contain the path to the list in which the
    //! item resides.

    let mut indices: Vec<usize> = Vec::new();

    while let Some(ind) = num_depth.next_if(|i| i.parse::<usize>().is_ok()) {
        indices.push(ind.parse::<usize>().unwrap());
    }

    indices.pop(); // Remove the back, because the back refers to the individual item

    indices
}

pub fn search_all_todos_content(todos_list: &Vec<Todo>, needle: &str) -> bool {
    for todo in todos_list {
        if todo.contents == needle || search_all_todos_content(&todo.sub_todos, needle) {
            return true;
        }
    }

    false
}
