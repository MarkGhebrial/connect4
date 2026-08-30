# `connect4`

No AI was used in the creation of this project.

This is an engine for playing connect 4. I started this project because I wanted to implement the algorithms used in chess engines, but without first having to implement the rules of chess. Connect 4 [is a solved game](https://en.wikipedia.org/wiki/Connect_Four#Mathematical_solution), but my engine is not (yet) capable of perfect play.

Things are split into a handful of crates:
- [`c4ui`](/c4ui) is a TUI that makes it possible to play against the engine, or to run automated games between two engines.
- [`c4i`](/c4i) is a library that makes it easy to drive connect 4 engines over io streams such as stdio and TCP.
- [`c4board`](/c4board) is a library that contains the data structures and classes used to represent the connect 4 board.
- [`c4engine`](/c4engine) contains the evaluation and search functions for deciding the best move. It's both a library and binary crate. The binary runs a c4i interface on stdio.

If you want to play against the engine, clone this repository and run
```
cargo r --release --bin c4ui -- play
```

# TODO
- `y;;yyryry;yyrryr;ryyrrr;;` results in the computer playing in column 1, which leads to an immediate loss. Fix that.
- Transposition tables