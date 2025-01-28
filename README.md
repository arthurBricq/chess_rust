# ♔ Chess Engine in Rust ♚

This repo contains the code of a **chess engine** fully written in Rust, from scratch, and the UI to interact with it. I
would like to keep expanding this project to create a Chess annotation software.

This is one of my many personal projects in Rust. And it is my second chess engine (*I had done the first
in [C++](https://github.com/arthurBricq/chess_cpp)*)

![](screenshot.png)

It features:

- Very compact representation of a chess-game (*1 game is represented with 8 bytes*)
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

- Engine
    - Parallel alpha-beta pruning
    - Engine Server to keep expanding while the other player is thinking

- Lichess API

What I am not happy about

- I don't really like the "redundancy" between the `attacks` module and the `get_next_moves`: when computing the next
  moves, we absolutely don't use the next pre-computed masks. I can probably optimize some things.
- I could come up with a better benchmarking system.
- All the `TODO` in the code that I have left.

### Castling

The very big challenge to implement castling is that you need to know which squares are attacked in a given position.
There are no easy ways to compute that optimally.

It is very easy to understand that the problem of implementing castling rules is equally hard as the problem to compute
attacked squares by a player. Most specifically, the player that is not currently playing.

I have implemented this computation using bitmasks that are precomputed before starting a game, and stored in memory.
The idea is quite simple, and must be differentiated between two kinds of pieces.

- For pawns, kights and kings: given every 64 possible positions, you always know the attacked positions. These can be
  precomputed and stored at all time.
- For bishops, rooks and queens: given every 64 possible positions, you always know the "rays" attacked. You can always
  go through the raw in a defined order until (1) the ray is finished or (2) you find a piece.

All of this logic is defined in the `attacks` module.

**Is it possible to "factorize" the computation of the attacked squares and the computation of the next moves ?**

No, it's not. Because when you compute the attacked squares, the player has not yet played. The fact that the player
applies 1 more move change everything.
