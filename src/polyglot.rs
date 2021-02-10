/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::structs::*;
use crate::statics::*;
use crate::uci::*;

//external modules:

/////////////////////////////////////////////////////////////////////

fn has_pawn_for_en_passant(board: &BoardStructure) -> bool {
    let target_piece = if board.side == Players::White {Pieces::Wp} else {Pieces::Bp};
    let attacker_sq: usize;

    if board.en_pas != Squares::NoSq as usize {
        if board.side == Players::White {
            attacker_sq = board.en_pas as usize - 10;
        } else {
            attacker_sq = board.en_pas as usize + 10;
        }

        if board.board[attacker_sq + 1] == target_piece {
            return true;
        } else if board.board[attacker_sq - 1] == target_piece {
            return true;
        }
    }
    
    // else:
    return false;
}

// from: http://hgm.nubati.net/book_format.html
pub fn polykey_from_board(board: &BoardStructure) -> u64 {
    
    let mut key: u64 = 0;

    let mut piece: Pieces;
    let mut poly_piece: usize;

    let mut rank: usize;
    let mut file: usize;

    for sq in 0..120 {
        piece = board.board[sq];

        if piece != Pieces::Empty && piece != Pieces::OffLimits {
            poly_piece = PIECE_TO_POLY_KIND[piece as usize];

            rank = RANKS_FROM_120[sq];
            file = FILES_FROM_120[sq];

            key ^= POLY_ZOBRIST_KEYS[(64 * poly_piece) + (8 * rank) + file]
        }
    }

    // castling
    if (board.castle_perm & 8) != 0 {key ^= POLY_ZOBRIST_KEYS[768 + 0]}
    if (board.castle_perm & 4) != 0 {key ^= POLY_ZOBRIST_KEYS[768 + 1]}
    if (board.castle_perm & 2) != 0 {key ^= POLY_ZOBRIST_KEYS[768 + 2]}
    if (board.castle_perm & 1) != 0 {key ^= POLY_ZOBRIST_KEYS[768 + 3]}


    // en passant
    if has_pawn_for_en_passant(board) {
        file = FILES_FROM_120[board.en_pas as usize];
        key ^= POLY_ZOBRIST_KEYS[772 + file]
    }

    // side (only apply when white)
    if board.side == Players::White {
        key ^= POLY_ZOBRIST_KEYS[780];
    }

    return key;
}

fn parse_polymove(action: u16, board: &BoardStructure) -> usize {
    let ff = FILE_CHARS[((action >> 6) & 7) as usize];
    let fr = RANK_CHARS[((action >> 9) & 7) as usize];
    let tf = FILE_CHARS[((action >> 0) & 7) as usize];
    let tr = RANK_CHARS[((action >> 3) & 7) as usize];

    let mut action_vec: Vec<char> = Vec::from([ff, fr, tf, tr]);

    // handle promotion
    let pp = ((action >> 12) & 7) as usize;
    match pp {
        0 => (),
        1 => action_vec.push('n'),
        2 => action_vec.push('b'),
        3 => action_vec.push('r'),
        4 => action_vec.push('q'),
        _ => panic!("polyglot promotion number malformed!")
    }

    let action_string = action_vec.into_iter().collect();

    return parse_move(action_string, board);
}

pub fn get_book_move(board: &BoardStructure, book: &PolyglotBook) -> usize {

    let mut book_recommends: Vec<(usize, u16)> = Vec::new();
    let board_key = polykey_from_board(board);

    let mut action: usize;

    
    for entry in book.entries.iter() {
        if (*entry).key == board_key {

            action = parse_polymove((*entry).action, board);

            if action != 0 {
                book_recommends.push( (action, (*entry).weigth) );
            }
        }
    }

    // sort bookmoves by valuation
    book_recommends.sort_by_key(|t| t.1);

    // return best move
    match book_recommends.get(0) {
        None         => return 0,
        Some(t)      => return t.0,
    }
}