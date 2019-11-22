extern crate rand;
extern crate shakmaty;
extern crate shakmaty_syzygy;
extern crate clap;

use shakmaty::{fen, san::San, Board, Chess, Color, FromSetup, Piece, Position, Role};
use shakmaty_syzygy::{Dtz, Tablebase, Wdl};

use rand::Rng;
use std::iter;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Only move generator")
        .version("0.1")
        .author("Morten Lohne")
        .about("Generate random positions where there is only move that wins, or one move that saves the draw.")
        .arg(Arg::with_name("syzygypath")
            .help("One or more paths to a directory containing syzygy tablebase. The full set is required, including wdl and dtz, and tablebases for less than n pieces.")
            .required(true)
            .multiple(true))
        .arg(Arg::with_name("n")
            .short("n")
            .help("Number of pieces to generate positions for. The program will only generate positions with exactly n pieces.")
            .default_value("6")
            .possible_values(&["3", "4", "5", "6", "7"]))
        .get_matches();

    let tb_files: Vec<_> = matches
        .values_of("syzygypath")
        .unwrap()
        .collect();

    let num_pieces: u8 = matches
        .value_of("n")
        .unwrap()
        .parse()
        .unwrap();

    let mut tables = Tablebase::new();

    for value in tb_files {
        tables.add_directory(value).unwrap();
    }

    let mut rng = rand::thread_rng();

    iter::repeat_with(|| generate_random_position_with_eval(&tables, &mut rng, num_pieces))
        .filter(|(_, _, dtz)| dtz.0.abs() > 10 && dtz.0.abs() < 100)
        .filter_map(|(pos, wdl, dtz)| {
            let mut children = pos
                .legals()
                .iter()
                .map(|mv| (mv.clone(), pos.clone().play(mv).unwrap()))
                .map(|(mv, child_pos)| (mv, tables.probe_dtz(&child_pos).unwrap()))
                .filter(|(_, child_dtz)| (child_dtz.0 + dtz.0).abs() <= 4)
                .collect::<Vec<_>>();

            children.sort_by_key(|(_, child_dtz)| child_dtz.0);
            children.reverse();

            debug_assert!(!children.is_empty(), "{}: {:?}", fen::fen(&pos), dtz);
            if children.len() > 1 && (children[1].1 + dtz).0.abs() > 2 {
                Some((pos, wdl, dtz, children[0].0.clone()))
            } else {
                None
            }
        })
        .for_each(|(pos, wdl, dtz, mv)| match wdl {
            Wdl::Win => println!(
                "{} hmvc {}; bm {}",
                fen::epd(&pos),
                100 - dtz.0 - 1,
                San::from_move(&pos, &mv).to_string()
            ),
            Wdl::Loss => println!(
                "{} hmvc {}; bm {}",
                fen::epd(&pos),
                100 - dtz.0.abs() + 2,
                San::from_move(&pos, &mv).to_string()
            ),
            _ => unreachable!(),
        });
}

fn generate_random_position_with_eval<R: Rng>(tables: &Tablebase<Chess>, rng: &mut R, num_pieces: u8) -> (Chess, Wdl, Dtz) {
    let pos: Chess = generate_random_position(rng, num_pieces);

    let wdl = tables.probe_wdl(&pos).unwrap();

    let dtz = tables.probe_dtz(&pos).unwrap();

    (pos, wdl, dtz)
}

fn generate_random_position<R: Rng>(rng: &mut R, num_pieces: u8) -> Chess {
    let mut board = Board::empty();

    // Set kings
    let square = shakmaty::Square::new(rng.gen_range(0, 64));
    board.set_piece_at(
        square,
        Piece {
            color: Color::White,
            role: Role::King,
        },
        false,
    );
    loop {
        let square = shakmaty::Square::new(rng.gen_range(0, 64));
        if board.color_at(square).is_none() {
            board.set_piece_at(
                square,
                Piece {
                    color: Color::Black,
                    role: Role::King,
                },
                false,
            );
            break;
        }
    }
    for _ in 2..num_pieces {
        let role = match rng.gen_range(0, 4) {
            0 => Role::Pawn,
            1 => Role::Knight,
            2 => Role::Bishop,
            3 => Role::Rook,
            4 => Role::Queen,
            _ => unreachable!(),
        };
        let color = if rng.gen() {
            Color::White
        } else {
            Color::Black
        };
        loop {
            let square = shakmaty::Square::new(rng.gen_range(0, 64));

            if board.color_at(square).is_none() {
                board.set_piece_at(square, Piece { color, role }, false);
                break;
            }
        }
    }

    let color = if rng.gen() {
        Color::White
    } else {
        Color::Black
    };

    let fen = shakmaty::fen::Fen {
        board,
        pockets: None,
        turn: color,
        castling_rights: shakmaty::Bitboard::EMPTY,
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: 1,
    };

    Chess::from_setup(&fen).unwrap_or_else(|_| generate_random_position(rng, num_pieces))
}
