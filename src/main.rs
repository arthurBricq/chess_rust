use crate::view::gtk_view::GTKView;

mod model;

pub mod view {
    pub mod chessview;
    // pub mod web_display; 
    pub mod terminal_display;
    pub mod gtk_view;
}

pub mod tests {
    pub mod test;
    pub mod test_engine;
    pub mod test_game_api;
}

fn play() {
    // Hierarchy problem:
    // At the moment, the view instantiate the view model which itself instantiate the game.
    // Everything is reversed.
    let mut my_view = GTKView::new();
    my_view.run_app();
}

fn main() {
    play();
}

