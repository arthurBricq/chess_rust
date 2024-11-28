mod engine;
mod model;

use crate::engine::engine::Engine;
use crate::engine::iterative_deepening::IterativeDeepeningEngine;
use crate::model::game::ChessGame;
use crate::model::moves::Move;
use std::time::Instant;

/// This function runs a benchmarking of the chess game
fn benchmark(mut game: ChessGame, folds: usize) {
    let mut times: Vec<f64> = Vec::new();

    for _i in 0..folds {
        // let mut engine = AlphaBetaEngine::new();
        // engine.set_engine_depth(7, 2);
        let engine = IterativeDeepeningEngine::new(8, 0);
        let mut solver: Box<dyn Engine> = Box::new(engine);
        let start = Instant::now();
        let result = solver.find_best_move(game, false);
        let best_move = result.best_move.unwrap();
        let _success = game.apply_move_safe(Move::new(best_move.from, best_move.to, false));
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
    // Start the game after e4, e5, Kf3, Kc6, d4
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
    benchmark(game, 10);
}
