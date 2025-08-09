use crate::alpha_beta::AlphaBetaEngine;
use crate::engine::{Engine, SearchResult};
use crate::iterative_deepening::IterativeDeepeningEngine;
use model::game::ChessGame;
use model::moves::Move;

/// Struct used when asserting a puzzle.
/// Puzzles often consists of a series of forced moved with 1 forced answer.
struct PuzzleAssert {
    expected_best_move: Move,
    puzzle_continuation: Option<Move>,
}

/// Given a puzzle, asserts that the engine finds all the best move.
fn solve_puzzle(
    mut engine: impl Engine,
    mut game: ChessGame,
    white_to_play: bool,
    expected_answers: &[PuzzleAssert],
) {
    game.block_castling();
    game.display();

    for PuzzleAssert {
        expected_best_move,
        puzzle_continuation,
    } in expected_answers
    {
        let SearchResult {
            score: _,
            best_move,
        } = engine.find_best_move(game, white_to_play);

        // Asserts that the engine is correct
        assert_eq!(Some(*expected_best_move), best_move);

        // Apply the puzzle continuation
        if let Some(forced_answer) = puzzle_continuation {
            game.apply_move_unsafe(&expected_best_move);
            println!("found: {expected_best_move}");
            game.display();
            game.apply_move_unsafe(&forced_answer);
            println!("answered by: {forced_answer}");
            game.display();
        }
    }
}

// https://lichess.org/training/giHum
// Themes: mates in two, double check
// Rating: 2021
#[test]
fn puzzle_1() {
    let game = ChessGame::from_fen("6r1/p1q3bk/4rnR1/2p2Q1P/1p1p4/3P2P1/2PK1B2/8 w - - 0 46");

    solve_puzzle(
        AlphaBetaEngine::new(6, 0),
        game,
        true,
        &[
            // Rooks sacrifices, forces the king in h7 to captures the rook in g7
            PuzzleAssert {
                expected_best_move: Move::from_str("g6", "h6", true),
                puzzle_continuation: Some(Move::from_str("h7", "h6", false)),
            },
            // Queen check mate in g6
            PuzzleAssert {
                expected_best_move: Move::from_str("f5", "g6", true),
                puzzle_continuation: None,
            },
        ],
    )
}

#[test]
fn practice_back_rank_mate_1() {
    let game = ChessGame::from_fen("6k1/4Rppp/8/8/8/8/5PPP/6K1 w - - 0 1");
    solve_puzzle(
        AlphaBetaEngine::new(6, 0),
        game,
        true,
        &[PuzzleAssert {
            expected_best_move: Move::from_str("e7", "e8", true),
            puzzle_continuation: None,
        }],
    )
}

#[test]
fn practice_back_rank_mate_2() {
    let game = ChessGame::from_fen("2r1r1k1/5ppp/8/8/Q7/8/5PPP/4R1K1 w - - 0 1");
    solve_puzzle(
        AlphaBetaEngine::new(6, 0),
        game,
        true,
        &[
            PuzzleAssert {
                expected_best_move: Move::from_str("e1", "e8", true),
                puzzle_continuation: Move::from_str("c8", "e8", false).into(),
            },
            PuzzleAssert {
                expected_best_move: Move::from_str("a4", "e8", true),
                puzzle_continuation: None,
            },
        ],
    )
}

/// This one is interesting: `AlphaBetaPrunning` works better than `IterativeDeepening`
///
/// What does this mean ? Is it even possible ?
/// It means that a wrong cut is introduced... I think that this should be strictly impossible,
/// let's try to understand it...
///
/// TODO: understand what is happening with this test.
#[test]
// #[ignore]
fn practice_back_rank_mate_3() {
    let game = ChessGame::from_fen("6k1/3qb1pp/4p3/ppp1P3/8/2PP1Q2/PP4PP/5RK1 w - - 0 1");
    solve_puzzle(
        IterativeDeepeningEngine::new(10, 0),
        game,
        true,
        &[
            PuzzleAssert {
                expected_best_move: Move::from_str("f3", "f7", true),
                puzzle_continuation: Move::from_str("g8", "h8", false).into(),
            },
            PuzzleAssert {
                expected_best_move: Move::from_str("f7", "f8", true),
                puzzle_continuation: Move::from_str("e7", "f8", false).into(),
            },
            PuzzleAssert {
                expected_best_move: Move::from_str("f1", "f8", true),
                puzzle_continuation: None,
            },
        ],
    )
}

#[test]
fn practice_hook_2_mate_in_3() {
    let game = ChessGame::from_fen("5r1b/2R1R3/P4r2/2p2Nkp/2b4N/6P1/4PP2/6K1 w - - 0 1");
    solve_puzzle(
        AlphaBetaEngine::new(7, 0),
        game,
        true,
        &[
            PuzzleAssert {
                expected_best_move: Move::from_str("e7", "g7", true),
                puzzle_continuation: Move::from_str("h8", "g7", false).into(),
            },
            PuzzleAssert {
                expected_best_move: Move::from_str("c7", "g7", true),
                puzzle_continuation: Move::from_str("f6", "g6", false).into(),
            },
            PuzzleAssert {
                expected_best_move: Move::from_str("g7", "g6", true),
                puzzle_continuation: None,
            },
        ],
    )
}

#[test]
fn practice_en_passant_mate_in_1() {
    let mut game = ChessGame::from_fen("k7/p3p3/3b1p2/2RP1p2/2R5/2BPP3/3K2B1/8 b - - 0 1");
    println!("game: {game:?}");
    // Black make a mistake and allows white to capture the pawn with en passant, also delivering check-mate
    game.apply_move_unsafe(&Move::from_str("e7", "e5", false));
    println!("game: {game:?}");
    solve_puzzle(
        AlphaBetaEngine::new(7, 0),
        game,
        true,
        &[
            PuzzleAssert {
                expected_best_move: Move::from_str("d5", "e6", true),
                puzzle_continuation: None
            },
        ],
    )
}
