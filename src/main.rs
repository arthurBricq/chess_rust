use crate::view::fltk_view::GTKView;

mod model;
mod view;
mod engine;

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

