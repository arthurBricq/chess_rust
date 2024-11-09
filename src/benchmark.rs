mod model;

use chess::engine::alpha_beta::AlphaBetaEngine;
use crate::model::game_constructor::GameConstructor;
use crate::model::moves::Move;
use std::time::Instant;
use crate::model::tools::chesspos_to_index;

/// This function runs a benchmarking of the chess game
fn benchmark() {
    let mut nodes_per_seconds: Vec<u128> = Vec::new();
    let mut times: Vec<f64> = Vec::new();
    let mut solver = AlphaBetaEngine::new();
    solver.set_engine_depth(7, 2);
    let n = 10;

    for _i in 0..n {
        let mut game = GameConstructor::standard_game();

        // Start the game after e4, e5, Kf3, Kc6, d4
        game.apply_move_unsafe(&Move::new(chesspos_to_index("e2").unwrap(), chesspos_to_index("e4").unwrap(), true));
        game.apply_move_unsafe(&Move::new(chesspos_to_index("e7").unwrap(), chesspos_to_index("e5").unwrap(), false));
        game.apply_move_unsafe(&Move::new(chesspos_to_index("g1").unwrap(), chesspos_to_index("f3").unwrap(), true));
        game.apply_move_unsafe(&Move::new(chesspos_to_index("b8").unwrap(), chesspos_to_index("c6").unwrap(), true));
        game.apply_move_unsafe(&Move::new(chesspos_to_index("d2").unwrap(), chesspos_to_index("d4").unwrap(), true));

        let start = Instant::now();
        let (result, nps) = solver.find_best_move(game, false);
        let best_move = result.best_move.unwrap();
        let _success = game.apply_move_safe(Move::new(best_move.from, best_move.to, false));
        nodes_per_seconds.push(nps);
        let end = start.elapsed().as_millis() as f64;
        times.push(end);
    }

    println!("-------------------");
    println!("BENCHMARKING RESULT");
    println!("-------------------");
    println!("Nodes per seconds: {nodes_per_seconds:?}");
    println!("Number of iterations: {n}");
    println!("Mean                : {} [k-nps]", nodes_per_seconds.iter().sum::<u128>() as f64 / nodes_per_seconds.len() as f64);
    println!("Mean time           : {} [ms]", times.iter().sum::<f64>() / times.len() as f64);
}

fn main() {
    benchmark()
}
