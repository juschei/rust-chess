/////////////////////////////////////////////////////////////////////

//internal imports:
use crate::statics::*;
use crate::structs::*;

//external imports:

/////////////////////////////////////////////////////////////////////


//generate hash for the board to allow three fold repetiition check
pub fn generate_position_hash(board: &BoardStructure) -> u64 {
    let mut piece: usize;
    let mut key: u64 = 0u64;

    for square in 0..120 {
        piece = board.board[square] as usize;
        if piece != (Pieces::OffLimits as usize) && piece != (Pieces::Empty as usize) {
            key ^= PIECE_KEYS[square][piece];
        }
    }

    if board.side == Players::White {
        key ^= *SIDE_KEY;
    }

    if board.en_pas != (Squares::NoSq as usize) {
        key ^= PIECE_KEYS[board.en_pas as usize][0];
    }

    key ^= CASTLING_KEYS[board.castle_perm as usize];

    return key;
}
