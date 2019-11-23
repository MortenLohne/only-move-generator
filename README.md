# Only-move-generator

A small command-line utility for generating chess endgame positions that are particularly good at exposing chess engine bugs involving the 50-move rule.

*Only-move-generator* utilizes syzygy [endgames tablebases](https://en.wikipedia.org/wiki/Endgame_tablebase) to generate positions where there is only move that wins within the 50-move rule, or one move that saves the draw due to the 50-move rule. Additionally, every position will also have a move that misses the win/draw by one move, and will have at least 10 ply until the 50-move reset.

In tablebase terms, every position generated is either:

* A win with exactly one winning move, and one or more moves that lead to a cursed win.
* A blessed loss with exactly one drawing move, and one or more moves that lead to a true loss.

## Getting started

### Prerequisites
The tool can generate positions with 3-7 pieces, depending on which tablebase you have installed. It requires a *full set* of syzygy tablebases, both wdl and dtz, including the tablebases for less than n pieces. They can be downloaded from a variety of sources, including [https://syzygy-tables.info/](https://syzygy-tables.info/). 

Storing the tablebases on an SSD is significantly faster.

### Usage

`only-move-generator [-n <number of pieces>] <syzygypath>...`

Note that it may not start outputting results immediately. 

### Output 

The resulting positions are continuously written to standard output in [Extended Position Description](https://en.wikipedia.org/wiki/Extended_Position_Description) for as long as the program runs. It includes `hmvc` (half-move counter), `bm` (best move) and `ce` (centipawn evaluation) tags. Centipawn evaluation will always be either 0 (blessed loss) or 10000 (win).

Example output, using 6-man tablebases:
````
2k1K3/2r5/8/R7/8/4n3/2n5/8 b - - hmvc 76; bm Nd4; ce 10000
4rk2/8/1n6/8/6PK/1N6/8/8 b - - hmvc 86; bm Nc4; ce 10000
8/K7/B1k5/7R/8/2q5/8/3n4 w - - hmvc 78; bm Rh6; ce 0
8/8/8/8/kb3R2/6K1/n7/4R3 b - - hmvc 78; bm Nc3; ce 0
k5n1/8/r7/5R2/8/b3K3/8/8 b - - hmvc 66; bm Kb7; ce 10000
````

### Compiling 
Compiling requires a [Rust compiler](https://www.rust-lang.org/tools/install). The project has dependencies hosted on crates.io, so using cargo (Included in a rustup install) is strongly recommended.

#### Build
`cargo build --release`

This will crate an executable in `target/release/`.

# License

This project is licensed under the GPLv3 (or any later version at your option). See the LICENSE.md file for the full license text.