# ♔ Chess Engine in Rust ♚

This repo contains the code of a **chess engine** fully written in Rust, from scratch, and the UI to interact with it. I
would like to keep expanding this project to create a Chess annotation software.

This is one of my many personal projects in Rust. And it is my second chess engine (*I had done the first
in [C++](https://github.com/arthurBricq/chess_cpp)*)

![](screenshot.png)

It features:

- All the rules of chess written in a very compact format (*1 game is represented with 8 bytes*)
- alpha-beta pruning
- move ordering to favor captures
- iterative deepening to improve move ordering

## Quick Start

The only required dependency is cargo.

```bash
git clone <...>
cd chess_rust
cargo run --bin chess --release
```

The flag `--release` tells rust to optimize the code (this is absolutely required), and `--features fltk` specify that
you want to compile the UI and to run with the UI.

## More words about the Engine

- Tree-search for the best move (min-max algorithm). The resulting computing speed is about 500'000 moves / second.
    - Alpha Beta pruning to do it faster
    - Transposition table to avoid double computation
    - Extra depth for captures move
- Evaluation function that favors attacking positions
- An extremely light-weight chess representation
- UI to play locally on your computer

The whole idea behind this chess engine is that the representation of a chess game is very small: it's **only** 7
integers !

```rust
pub struct ChessGame {
    whites: u64,
    pawns: u64,
    bishops: u64,
    knights: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
    flags: u64
}
```

This was a design decision, that has benefits and drawbacks.

- Drawbacks: it's a bit hard to represent moves, captures, ...
- Benefits: It's super quick to copy a game, therefore the solver can just create massive amounts of games instead of
  having to work with a single game.

This code contains the following part:

- The **representation of a chess game** (position of pieces, rules to determine if moves are valid), under
  `model/game.rs` and `model/moves.rs`. This part is really the most difficult part.
- The **chess engine**. Basically, it's a recursive function that explores the **tree of possible moves** until a given
  depth. The tree-serach algorithm is a **min-max** search with **alpha-beta prunning**. The engine is located under
  `model/engine.rs`
- To play with this engine, I have developed a **UI** very easy to deploy with FTLK.rs. There is also a WebApp (because
  i wanted to try `yew.rs`) in the eventual possibility to host this engine on my website...

## Benchmarking result

The command `cargo run` without any features will run the benchmarking test.

## [Dev] TODO List for myself

List of missing features

- Chess Rules
    - [ ] En Passant, using a stateful representation
    - [ ] Non-queen promotions
    - [ ] Proper castling

- Engine
    - Parallel alpha-beta pruning
    - Engine Server to keep expanding while the other player is thinking

### Castling

The very big challenge to implement castling is that you need to know which squares are attacked in a given position.
There are no easy ways to compute that optimally.

You could think that there is a strong link between the attacked squares and the possible moves, and this is true for
most pieces except for pawns.

My goal is to get the following workflow

- each game keeps track of all the attacked positions, as one new field `attacked_positions: u64`
    - this will solve the castling rules
    - this will allow to give a better evaluation function: you can ponderate the score with the number of attacked
      squares
- computing `attacked_positions` must also compute the possible `moves` in this position (of the next player)

I try to make a roadmap here

- Part I: Given a chess position, compute all the attacked pieces. For now, I want to recreate this from scratch (
  without using my `next moves` computation)
    - bit mask
        - [x] Implement pawn attack using bit mask
        - [ ] Implement knight attack using bit mask
        - [ ] Implement king attack using bit mask
    - sliding pieces using "ray mask"
- Part II: Now that we have attacked positions, implement castling rules. This will obviously slow down a lot the
  engine.
    - add `attacked_position` field and compute it automatically after applying moves
    - implement castling rules
- Part III: factorize computation of `next move`, somehow... The goal of this step is to "restore" the old speed of the
  chess engine. This will not be entirely possible, but you have to keep in mind that the chess evaluation function will
  benefit from this work.

### How to optimise the score function ?

- solver calls game.getAllMoves --> we know the size here.
- for each move, create a new game
- for each new game, call the solver : if finished, call game.score()

==> for a given game, game.score() is always called before game.getAllMoves ... 
