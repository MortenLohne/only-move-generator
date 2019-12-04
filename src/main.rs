extern crate clap;
extern crate rand;
extern crate shakmaty;
extern crate shakmaty_syzygy;

mod cli;
mod position_generator;

use std::io::ErrorKind;
use std::iter;

use shakmaty::{fen, san::San};
use shakmaty_syzygy::{Tablebase, Wdl};

fn main() {
    let cli_options = cli::parse_cli_arguments();

    let mut tables = Tablebase::new();

    for tb_path in &cli_options.tb_file_names {
        tables.add_directory(tb_path).unwrap_or_else(|err| {
            match err.kind() {
                ErrorKind::NotFound => eprintln!("Couldn't find {}: {}", tb_path, err),
                _ => eprintln!("Error: Failed to open {}: {:?}", tb_path, err),
            }
            std::process::exit(66);
        });
    }

    let mut rng = rand::thread_rng();

    iter::repeat_with(|| {
        position_generator::generate_random_position_with_eval(
            &tables,
            &mut rng,
            cli_options.num_pieces,
        )
    })
    .filter(|(_, _, dtz)| dtz.0.abs() > cli_options.dtz_minimum as i32 && dtz.0.abs() < 100)
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
