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
    let mut view = GTKView::new();
    view.run_app();
}

// #[cfg(all(not(feature = "fltk"), not(feature = "benchmark")))]
// fn play() {
//     use view::terminal_display::TerminalChessView;
//
//     let mut game = ChessGame::standard_game();
//     let mut solver = Engine::new();
//     if let (result, _nps) = solver.find_best_move(game, false) {
//         if let Some(best_move) = result.best_move {
//             game.apply_move_safe(
//                 Move::new(best_move.from, best_move.to, false)
//             );
//         }
//     }
//
//     let mut view = TerminalChessView::new(&mut game);
//     view.display();
// }


fn main() {
    play();
}
