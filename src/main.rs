extern crate rand;
extern crate shakmaty;
extern crate shakmaty_syzygy;

use shakmaty::{fen, Board, Chess, Color, FromSetup, Piece, Position, Role};
use shakmaty_syzygy::{Dtz, Tablebase, Wdl};

use rand::Rng;
use std::iter;

fn main() {
    let mut tables = Tablebase::new();
    tables
        .add_directory("/media/morten/Shared data/Syzygy/6man/dtz")
        .unwrap();
    tables
        .add_directory("/media/morten/Shared data/Syzygy/6man/wdl")
        .unwrap();

    tables
        .add_directory("/media/morten/Shared data/Syzygy/syzygy")
        .unwrap();

    let mut rng = rand::thread_rng();

    iter::repeat_with(|| generate_6man_with_eval(&tables, &mut rng))
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
            Wdl::Win => println!("{} hmvc {}; bm {}", fen::epd(&pos), 100 - dtz.0, mv),
            Wdl::Loss => println!(
                "{} hmvc {}; bm {}",
                fen::epd(&pos),
                100 - dtz.0.abs() + 1,
                mv
            ),
            _ => unreachable!(),
        });
}

fn generate_6man_with_eval<R: Rng>(tables: &Tablebase<Chess>, rng: &mut R) -> (Chess, Wdl, Dtz) {
    let pos: Chess = generate_random_6man(rng);

    let wdl = tables.probe_wdl(&pos).unwrap();

    let dtz = tables.probe_dtz(&pos).unwrap();

    (pos, wdl, dtz)
}

fn generate_random_6man<R: Rng>(rng: &mut R) -> Chess {
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
    for _ in 0..4 {
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

    Chess::from_setup(&fen).unwrap_or_else(|_| generate_random_6man(rng))
}
