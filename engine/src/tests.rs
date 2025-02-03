use crate::engine::{Engine, SearchResult};
use crate::iterative_deepening::IterativeDeepeningEngine;
use model::game_constructor::GameConstructor;
use model::moves::Move;
use crate::alpha_beta::AlphaBetaEngine;

struct PuzzleAssert {
    expected_best_move: Move,
    puzzle_continution: Option<Move>,
}

/// Given a puzzle, asserts that the engine finds all the best move.
fn solve_puzzle(
    mut engine: impl Engine,
    fen: &str,
    white_to_play: bool,
    expected_answers: &[PuzzleAssert],
) {
    let mut game = GameConstructor::from_fen(fen);
    game.display();

    for PuzzleAssert {
        expected_best_move,
        puzzle_continution: puzzle_continuation,
    } in expected_answers
    {
        let SearchResult { score, best_move } = engine.find_best_move(game, white_to_play);

        // Asserts that the engine is correct
        assert_eq!(best_move, Some(*expected_best_move));

        // Apply the puzzle continuation
        if let Some(puzzle_continution) = puzzle_continuation {
            game.apply_move_unsafe(&expected_best_move);
            game.display();
            game.apply_move_unsafe(&puzzle_continution);
            game.display();
        }
    }
}

// https://lichess.org/training/giHum
// Themes: mates in two, double check
// Rating: 2021
#[test]
fn mate_in_two_a() {
    let fen = "6r1/p1q3bk/4rnR1/2p2Q1P/1p1p4/3P2P1/2PK1B2/8 w - - 0 46";

    solve_puzzle(
        // IterativeDeepeningEngine::new(4, 4),
        AlphaBetaEngine::new(),
        fen,
        true,
        &[
            // Rooks sacrifices, forces the king in h7 to captures the rook in g7
            PuzzleAssert {
                expected_best_move: Move::from_str("g7", "g7", true),
                puzzle_continution: Some(Move::from_str("h7", "g7", false)),
            },
            // Queen check mate in g6
            PuzzleAssert {
                expected_best_move: Move::from_str("f5", "g6", true),
                puzzle_continution: None,
            },
        ],
    )
}
