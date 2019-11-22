# Only-move-generator

A small command-line utility for generating chess endgame positions that are particularly good at exposing chess engine bugs involving the 50-move rule.

*Only-move-generator* utilizes syzygy [endgames tablebases](https://en.wikipedia.org/wiki/Endgame_tablebase) to generate positions where there is only move that wins within the 50-move rule, or one move that saves the draw due to the 50-move rule. Additionally, every position will also have a move that misses the win/draw by one move.

In tablebase terms, every position generated is either:

* A win with exactly one winning move, and one or more moves that lead to a cursed win.
* A blessed loss with exactly one drawing move, and one or more moves that lead to a true loss.

The resulting positions are continuously written to standard output in [Extended Position Description](https://en.wikipedia.org/wiki/Extended_Position_Description) for as long as the program runs. It includes `hmvc` (half-move counter), `bm` (best move) and `ce` (centipawn evaluation) tags. Centipawn evaluation will always be either 0 (blessed loss) or 10000 (win).

## Getting started

### Prerequisites
The tool can generate positions with 3-7 pieces, depending on which tablebase you have installed. It requires a *full set* of syzygy tablebases, both wdl and dtz, including the tablebases for less than n pieces. They can be downloaded from a variety of sources, including [https://syzygy-tables.info/](https://syzygy-tables.info/). 

Storing the tablebases on an SSD is significantly faster.

### Usage

`only-move-generator [-n <number of pieces>] <syzygypath>...`

Note that it may not start outputting results immediately. 

### Compiling 
Compiling requires a [Rust compiler](https://www.rust-lang.org/tools/install). The project has dependencies hosted on crates.io, so using rustup+cargo is strongly recommended.

#### Build
`cargo build --release`

#### Run
`cargo run --release`

# License

This project is licensed under the GPLv3 (or any later version at your option). See the LICENSE.md file for the full license text.