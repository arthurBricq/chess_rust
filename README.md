# ♔ Chess Engine in Rust ♚

This repo contains the code of a **chess engine** fully written in Rust, from scratch. This was a personal project to learn Rust, since the final result is quite satisfying, I decided to publish it.

This code contains the following part: 
- The **representation of a chess game** (position of pieces, rules to determine if moves are valid), under `model/game.rs` and `model/moves.rs`. This part is really the most difficult part.
- The **chess engine**. Basically, it's a recursive function that explores the **tree of possible moves** until a given depth. The tree-serach algorithm is a **min-max** search with **alpha-beta prunning**. The engine is located under `model/engine.rs`
- To play with this engine, I have developed a **UI** very easy to deploy with FTLK.rs. There is also a WebApp (because i wanted to try `yew.rs`) in the eventual possibility to host this engine on my website... 

## Quick Start 

The only required dependency is Rust with cargo.

```bash
git clone <...>
cd chess_rust
cargo run --release --features fltk
```

The flag `--release` tells rust to optimize the code (this is absolutely required), and `--features fltk` specify that you want to compile the UI and to run with the UI. 



## Why a chess engine to learn Rust ? 

Programming a chess engine is a brilliant exercise for an accomplished developer. I think that chess programming is really the perfect project for testing your own skills. 
- It is **easy enough** to not take too long: it can be done in a few weeks / months, which is good for self-projects. I often have the problem to start projects too long and I end up tired of them.
- It is **difficult enough** to really challenge you. There are some things that are really not trivial. Encoding the rule of chess in a way to allows to have a depth of search good enough is really not easy.


## [Dev] TODO List for 

List of missing features
- rules
    - [x] Pawns
    - [x] Bishop, Knight, Rook, Queen, King
    - [x] Castling
    - [ ] En Passant, using a stateful representation
    - [ ] Non-queen promotions
- Optimisation
    - [ ] Profile the code to see weak points, using flamegraph 
    - [ ] Benchmarking to see if something is better
- solver
    - [x] Basic tree search without prunning
    - [x] Alpha-Beta Prunning
    - [x] More aware score function
    - [ ] Move ordering
    - [ ] Extra depth search for interesting moves
- view
    - [x] Conditional compiling of the view
    - [ ] Better ViewModel <--> View architecture
    - [ ] Improve the terminal view (to be able to run on Linux without X server)


### How to optimise the score function ? 

- solver calls game.getAllMoves --> we know the size here.
- for each move, create a new game 
- for each new game, call the solver : if finished, call game.score()

==> for a given game, game.score() is always called before game.getAllMoves ... 



## Representation of a chess game using integers only

...peed

## Benchmarking result

The command `cargo run` without any features will run the benchmarking test.



