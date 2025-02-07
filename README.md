# ♔ Chess Engine in Rust ♚

This repo contains the code of a **chess engine** fully written in Rust, from scratch, and the UI to interact with it. I
would like to keep expanding this project to create a Chess annotation software.

This is one of my many personal projects in Rust. And it is my second chess engine (*I had done the first
in [C++](https://github.com/arthurBricq/chess_cpp)*)

![](screenshot.png)

It features:

- Very compact representation of a chess-game (*1 game is represented with 8 bytes*)
- Alpha-beta pruning + iterative deepening engine
- Move ordering to favor captures
- Compatible with the `lichess-bot`, so that I can play as a bot online, [read more here](./lichess_bot/README.md)

You can also play with a UI locally,

```bash
cargo run --bin chess --release
```

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

## Ideas

- Compute moves that deliver checks and use this in the move ordering.
- Maybe I could do a better transposition table that is able to "shortcut" branches to go deeper. At the moment, my transposition table only works at the maximum depth. 

### Missing features

- Chess Rules
    - [ ] En Passant, using a stateful representation
    - [ ] Non-queen promotions

- Engine
    - Parallel alpha-beta pruning
    - Engine Server to keep expanding while the other player is thinking

