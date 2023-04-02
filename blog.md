# Writing a chess engine in Rust

## UI 

Creating a good UI is a key part of showing off your work. 

I tried for several days to work with `iced-rs` but ended up highly unsatisfied with stage of this library. The basic idea seems very attracting, it provides a UI library that forces the user to respect good programming practice : to separate the view from the model. But the library is far from being mature enough for writing difficult apps. 

What mostly annoyed me was the difficulty to create custom styles. The styling is very difficult to approach, and there is little to no existing examples to get inspired from.

0  1  2  3  4  5  6  7
8
16
24
32
40
48 
56 57 58 59 60 61 62 63


## Alpha-Beta Prunning

The algorithm maintains two values, alpha and beta, which respectively represent: 
- alpha = the minimum score that the maximizing player (white) is assured of. Initially, negative infinity
- beta  = the maximum score that the minimizing player (black) is assured of. Initially, positive infinity

Idea: 
- when beta < alpha: whenever the maximum score that black is assured of, becomes less than the minimum score that white is assured of, then white can stop considering more moves