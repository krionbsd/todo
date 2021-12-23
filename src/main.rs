use crate::data::TodoItem;
use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;

mod view;
use view::build_ui;

pub fn main() {
    let todos = vec![TodoItem::new("1st todo"), TodoItem::new("2nd todo")];
    let initial_state = AppState::new(todos);

    let main_window = WindowDesc::new(build_ui)
        .title("Todo Tutorial")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
