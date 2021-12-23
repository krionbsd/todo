use druid::{im::Vector, Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    todos: Vector<TodoItem>,
}

impl AppState {
    pub fn new(todos: Vector<TodoItem>) -> Self {
        Self {
            todos: Vector::from(todos),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct TodoItem {
    done: bool,
    text: String,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            done: false,
            text: text.into(),
        }
    }
}
