use crate::engine::{Engine, SearchResult};
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
        let SearchResult { score: _, best_move } = engine.find_best_move(game, white_to_play);

        // Asserts that the engine is correct
        assert_eq!(Some(*expected_best_move), best_move);

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
fn puzzle_1() {
    let fen = "6r1/p1q3bk/4rnR1/2p2Q1P/1p1p4/3P2P1/2PK1B2/8 w - - 0 46";

    solve_puzzle(
        AlphaBetaEngine::new(),
        fen,
        true,
        &[
            // Rooks sacrifices, forces the king in h7 to captures the rook in g7
            PuzzleAssert {
                expected_best_move: Move::from_str("g6", "h6", true),
                puzzle_continution: Some(Move::from_str("h7", "h6", false)),
            },
            // Queen check mate in g6
            PuzzleAssert {
                expected_best_move: Move::from_str("f5", "g6", true),
                puzzle_continution: None,
            },
        ],
    )
}

#[test]
fn practice_back_rank_mate_1() {
    solve_puzzle(
        AlphaBetaEngine::new(),
        "6k1/4Rppp/8/8/8/8/5PPP/6K1 w - - 0 1",
        true,
        &[
            PuzzleAssert {
                expected_best_move: Move::from_str("e7", "e8", true),
                puzzle_continution: None,
            },
        ],
    )
}

#[test]
fn practice_back_rank_mate_2() {
    solve_puzzle(
        AlphaBetaEngine::new(),
        "2r1r1k1/5ppp/8/8/Q7/8/5PPP/4R1K1 w - - 0 1",
        true,
        &[
            PuzzleAssert {
                expected_best_move: Move::from_str("e1", "e8", true),
                puzzle_continution: Move::from_str("c8", "e8", false).into(),
            },
            PuzzleAssert {
                expected_best_move: Move::from_str("a4", "e8", true),
                puzzle_continution: None
            },
        ],
    )
}

#[test]
fn practice_back_rank_mate_3() {
    solve_puzzle(
        AlphaBetaEngine::new(),
        "6k1/3qb1pp/4p3/ppp1P3/8/2PP1Q2/PP4PP/5RK1 w - - 0 1",
        true,
        &[
            PuzzleAssert {
                expected_best_move: Move::from_str("f3", "f7", true),
                puzzle_continution: Move::from_str("g8", "h8", false).into(),
            },
            PuzzleAssert {
                expected_best_move: Move::from_str("f7", "f8", true),
                puzzle_continution: Move::from_str("e7", "f8", false).into(),
            },
            PuzzleAssert {
                expected_best_move: Move::from_str("f1", "f8", true),
                puzzle_continution: None
            },
        ],
    )
}
