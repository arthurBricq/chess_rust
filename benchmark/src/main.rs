use std::time::Instant;
use engine::engine::Engine;
use engine::iterative_deepening::IterativeDeepeningEngine;
use model::game::ChessGame;
use model::moves::Move;

/// Finds the best move at the given position, `folds` times and prints the average time spent on this position
fn benchmark(game: ChessGame, folds: usize, is_white: bool, depth: usize) {
    let mut times: Vec<f64> = Vec::new();

    for _i in 0..folds {
        // let mut engine = AlphaBetaEngine::new();
        // engine.set_engine_depth(7, 2);
        let mut copied_game = game.clone();
        let engine = IterativeDeepeningEngine::new(depth, 0);
        let mut solver: Box<dyn Engine> = Box::new(engine);
        let start = Instant::now();
        let result = solver.find_best_move(copied_game, false);
        let best_move = result.best_move.unwrap();
        let _success =
            copied_game.apply_move_safe(Move::new(best_move.from, best_move.to, is_white));
        let end = start.elapsed().as_millis() as f64;
        times.push(end);
    }

    println!("-------------------");
    println!("BENCHMARKING RESULT");
    println!("-------------------");
    println!("Number of iterations: {folds}");
    println!(
        "Mean time           : {} [ms]",
        times.iter().sum::<f64>() / times.len() as f64
    );
}

fn main() {
    // 1. Run the engine in an opening with all pieces
    // Resulting position after e4, e5, Kf3, Kc6, d4
    let game = ChessGame::new(
        4398451320767,
        67272588556035840,
        2594073385365405732,
        4611690416475996162,
        9295429630892703873,
        576460752303423496,
        1152921504606846992,
        0,
    );
    benchmark(game, 10, false, 8);
    /*
     */

    // 2. Run the engine in an end-game
    /*
    let mut game = ChessGame::empty();

    game.set_piece(King, true, "d1");
    game.set_piece(King, false, "d8");

    game.set_piece(Pawn, true, "c2");
    game.set_piece(Pawn, true, "d2");
    game.set_piece(Pawn, true, "e2");
    game.set_piece(Rook, true, "a1");
    game.set_piece(Knight, true, "b1");
    game.set_piece(Knight, true, "c1");

    game.set_piece(Pawn, false, "c7");
    game.set_piece(Pawn, false, "d7");
    game.set_piece(Pawn, false, "e7");
    game.set_piece(Rook, false, "f8");
    game.set_piece(Bishop, false, "a8");
    game.set_piece(Bishop, false, "b8");
    benchmark(game, 10, false, 8);
     */
}
