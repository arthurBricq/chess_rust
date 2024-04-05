pub mod model {
    pub mod game;
    pub mod moves;
    pub mod engine;
}

pub mod view {
    pub mod chessview;
    // pub mod web_display; 
    pub mod terminal_display;
    #[cfg(feature = "fltk")]
    pub mod gtk_view;
}

pub mod tests {
    pub mod test;
    pub mod test_engine;
}

use model::game::*;
use model::engine::Engine;
use model::moves::Move;

#[cfg(all(feature = "fltk", not(feature = "benchmark")))]
fn play() {
    use crate::view::gtk_view::*;
    // Hierarchy problem:
    // At the moment, the view instantiate the view model which itself instantiate the game.
    // Everything is reversed.
    let mut view = GTKView::new();
    view.run_app();
}

#[cfg(all(not(feature = "fltk"), not(feature = "benchmark")))]
fn play() {
    use view::terminal_display::TerminalChessView;

    let mut game = ChessGame::standard_game();
    let mut solver = Engine::new();
    if let (Some(best_move), _nps) = solver.find_best_move(game, false) {
        let _success = game.apply_move_safe(
            Move::new(best_move.from, best_move.to)
        );
    }

    let mut view = TerminalChessView::new(&mut game);
    view.display();
}

#[cfg(feature = "benchmark")]
fn play() {
    benchmark();
}

/// This function runs a benchmarking of the chess game
fn benchmark() {
    let mut nodes_per_seconds: Vec<u128> = Vec::new();
    let mut solver = Engine::new();
    let n = 20;

    for _i in 0..n {
        let mut game = ChessGame::standard_game();
        if let (Some(best_move), nps) = solver.find_best_move(game, false) {
            let _success = game.apply_move_safe(
                Move::new(best_move.from, best_move.to)
            );
            nodes_per_seconds.push(nps);
        }
    }

    println!("-------------------");
    println!("BENCHMARKING RESULT");
    println!("-------------------");
    println!("Nodes per seconds: {nodes_per_seconds:?}");
    println!("Number of iterations: {n}");
    println!("Mean: {} [k-nps]", nodes_per_seconds.iter().sum::<u128>() as f64 / nodes_per_seconds.len() as f64);
}

fn main() {
    play();
}
