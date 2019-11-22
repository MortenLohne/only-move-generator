use rand::Rng;
use shakmaty::{fen, Board, Chess, Color, FromSetup, Move, Piece, Position, Role};
use shakmaty_syzygy::{Dtz, Tablebase, Wdl};

/// Returns the move with lowest depth to zero, or None if there are several moves with the same dtz.
pub fn get_single_best_reply(tables: &Tablebase<Chess>, pos: &Chess, dtz: Dtz) -> Option<Move> {
    let mut children = pos
        .legals()
        .iter()
        .map(|mv| (mv.clone(), pos.clone().play(mv).unwrap()))
        .map(|(mv, child_pos)| (mv, tables.probe_dtz(&child_pos).unwrap()))
        .filter(|(_, child_dtz)| (child_dtz.0 + dtz.0).abs() <= 4)
        .collect::<Vec<_>>();

    children.sort_by_key(|(_, child_dtz)| child_dtz.0);
    children.reverse();

    debug_assert!(!children.is_empty(), "{}: {:?}", fen::fen(pos), dtz);
    if children.len() > 1 && (children[1].1 + dtz).0.abs() > 2 {
        Some(children[0].0.clone())
    } else {
        None
    }
}

pub fn generate_random_position_with_eval<R: Rng>(
    tables: &Tablebase<Chess>,
    rng: &mut R,
    num_pieces: u8,
) -> (Chess, Wdl, Dtz) {
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
