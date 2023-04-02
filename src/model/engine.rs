use super::super::model::game::*;
use super::super::model::moves::*;
use std::time::Instant;

const ENGINE_DEPTH: i8 = 2; 

type SearchResult = (ScoreType, Option<Move>); 

pub struct Engine {
    iter: u64
}

impl Engine {
    pub fn new() -> Self {
        Self { iter: 0 }
    }

    /// For a given chessgame, finds the solver's best move and returns it as an Option of a move. 
    /// The function also returns the NPS (nodes per second) in the unit k-nps (for benchmarking)
    pub fn find_best_move(&mut self, game: &ChessGame, white_to_play: bool) -> (Option<Move>, u128) {
        self.iter = 0; 
        let start = Instant::now();
        let result = self.tree_search(game, white_to_play, 0, ScoreType::MIN, ScoreType::MAX, false); 
        let end = start.elapsed().as_millis() ;
        let nps = (self.iter as u128) / end; 


        // println!("Best move found, score = {}", result.0); 
        // println!("Number of positions evaluated ? {}", self.iter);
        // println!("Elapsed time: {:?} [ms]", end);
        // println!("Kilo-Nodes per second: {nps} k-nps"); 

       return (result.1, nps); 
    }

    fn tree_search(&mut self, 
        game: &ChessGame, 
        white_to_play: bool, 
        depth: i8, 
        mut alpha: ScoreType, 
        mut beta: ScoreType,
        last_move_captured: bool,
    ) -> SearchResult {

        // Ending condition
        if depth == 2*ENGINE_DEPTH {
            self.iter += 1; 
            return (game.score(), None) ; 
        } 

        // if (depth == 2*ENGINE_DEPTH && !last_move_captured) || (depth == 4*ENGINE_DEPTH)  {
        //     self.iter += 1; 
        //     return (game.score(), None) ; 
        // } 

        // GOAL 
        let mut current_score = if white_to_play {
            ScoreType::MIN
        } else {
            ScoreType::MAX
        } ;

        // The best move is initialized with the first one
        let mut best_move = 0; 

        // get the list of available moves
        // this is the only time that the function is called
        let moves = game.get_avalaible_moves(white_to_play); 

        // - (eventually) sort the list to favorize captures 

        // for each available move that is also valid, apply this move and run the search to the next depth
        for i in 0..moves.len() {
            // make a copy of the game
            let mut new_game = (*game).clone(); 

            // apply the move on the copy, without any kind of check 
            // (because we only have valid moves)
            new_game.apply_move_unsafe(&moves[i]); 

            // call the recursion
            let result = self.tree_search(&new_game, !white_to_play, depth+1, 
                alpha, 
                beta, 
                moves[i].is_capture()); 

            // Update the scores
            if white_to_play {
                if result.0 > current_score {
                    best_move = i; 
                    current_score = result.0; 
                }
                if result.0 > beta {
                    // stop exploring
                    break; 
                }
                if result.0 > alpha {
                    alpha = result.0; 
                }
            } else {
                if result.0 < current_score {
                    best_move = i; 
                    current_score = result.0; 
                }
                if result.0 < alpha {
                    // stop exploring
                    break; 
                }
                if result.0 < beta {
                    beta = result.0; 
                }
            }

            // Debug print
            // let indent = 3*depth as usize; 
            // println!("{:indent$}{}", "", format!("[{}], move {}, score={}, current_score={current_score}", white_to_play, moves[i], result.0), indent=indent);
        }

        // Once we reach this point, we know which is the best move. We return the score.

        // let indent = 3*depth as usize; 
        // println!("{:indent$}{}", "", format!("returning score-->{current_score}"), indent=indent);

        if depth == 0 {
            return (current_score, Some(moves[best_move])); 
        } else {
            return (current_score, None); 
        }
    }
}
