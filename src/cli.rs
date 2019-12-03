use clap::{App, Arg};

pub struct CliOptions {
    pub tb_file_names: Vec<String>,
    pub num_pieces: u8,
    pub dtz_minimum: u16,
}

pub fn parse_cli_arguments() -> CliOptions {
    let matches = App::new("Only move generator")
        .version("0.2.0")
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

    CliOptions {
        tb_file_names: matches
            .values_of("syzygypath")
            .unwrap()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>(),
        num_pieces: matches
            .value_of("number of pieces")
            .unwrap()
            .parse()
            .unwrap(),
        dtz_minimum: matches
            .value_of("minimum dtz")
            .unwrap()
            .parse()
            .unwrap_or_else(|err| {
                eprintln!("Failed to parse argument to dtz-minimum: {:?}", err);
                std::process::exit(64);
            }),
    }
}