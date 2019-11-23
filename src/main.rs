extern crate clap;
extern crate rand;
extern crate shakmaty;
extern crate shakmaty_syzygy;

mod position_generator;

use shakmaty::{fen, san::San};
use shakmaty_syzygy::{Tablebase, Wdl};

use clap::{App, Arg};
use std::iter;

fn main() {
    let matches = App::new("Only move generator")
        .version("0.1")
        .author("Morten Lohne")
        .about("Generate random positions where there is only move that wins, or one move that saves the draw.\nEvery position will also have a move that misses the win/draw by one move.")
        .arg(Arg::with_name("syzygypath")
            .help("One or more paths to a directory containing syzygy tablebase. The full set of wdl and dtz files is required, including the tablebases for less than n pieces.")
            .required(true)
            .multiple(true))
        .arg(Arg::with_name("number of pieces")
            .short("n")
            .help("Number of pieces to generate positions for. Only positions with exactly n pieces will be generated.")
            .default_value("6")
            .possible_values(&["3", "4", "5", "6", "7"]))
        .arg(Arg::with_name("minimum dtz")
            .short("d")
            .help("Lowest possible dtz in generated positions. Increasing this value makes the positions even more difficult.")
            .default_value("10"))
        .get_matches();

    let tb_file_names: Vec<_> = matches.values_of("syzygypath").unwrap().collect();

    let num_pieces: u8 = matches
        .value_of("number of pieces")
        .unwrap()
        .parse()
        .unwrap();

    let dtz_minimum: u16 = matches
        .value_of("minimum dtz")
        .unwrap()
        .parse()
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse argument to dtz-minimum: {:?}", err);
            std::process::exit(64);
        });

    let mut tables = Tablebase::new();

    for value in tb_file_names {
        tables.add_directory(value).unwrap();
    }

    let mut rng = rand::thread_rng();

    iter::repeat_with(|| {
        position_generator::generate_random_position_with_eval(&tables, &mut rng, num_pieces)
    })
    .filter(|(_, _, dtz)| dtz.0.abs() > dtz_minimum as i32 && dtz.0.abs() < 100)
    .filter_map(|(pos, wdl, dtz)| {
        let best_reply = position_generator::get_single_best_reply(&tables, &pos, dtz);
        best_reply.map(|reply| (pos, wdl, dtz, reply))
    })
    .for_each(|(pos, wdl, dtz, mv)| match wdl {
        Wdl::Win => println!(
            "{} hmvc {}; bm {}; ce 10000",
            fen::epd(&pos),
            100 - dtz.0 - 1,
            San::from_move(&pos, &mv).to_string()
        ),
        Wdl::Loss => println!(
            "{} hmvc {}; bm {}; ce 0",
            fen::epd(&pos),
            100 - dtz.0.abs() + 2,
            San::from_move(&pos, &mv).to_string()
        ),
        _ => unreachable!(),
    });
}
