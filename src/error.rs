#[derive(Debug)]
pub enum WhatodoError {
    IndexOutOfBounds,
    CannotLoadTodos(std::io::Error),
    CannotInitTodos(std::io::Error),
    TodoAlreadyInList,
    CannotSaveTodos(std::io::Error),
}

impl std::fmt::Display for WhatodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexOutOfBounds => write!(f, "Index out of bounds, could not perform operation"),
            Self::CannotLoadTodos(e) => write!(f, "Could not load todos: {e}"),
            Self::CannotInitTodos(e) => write!(f, "Could not init whatodo: {e}"),
            Self::TodoAlreadyInList => write!(f, "Todo is already in list, could not add todo"),
            Self::CannotSaveTodos(e) => write!(f, "Could not save todos: {e}"),
        }
    }
}

impl std::error::Error for WhatodoError {}
