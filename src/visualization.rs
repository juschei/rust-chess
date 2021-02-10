/////////////////////////////////////////////////////////////////////

//internal imports
use crate::bitboards::*;
use crate::statics::*;
use crate::structs::*;
use crate::legal_move_gen::*;
use crate::polyglot::*;

//external imports

/////////////////////////////////////////////////////////////////////

pub fn vis_bts() {
    //visualization of the 12x10 -> 8x8 conversion table
    for sq in 0..120 {
        if sq % 10 == 0 {
            println!("");
        }
        print!("{} ", BIG_TO_SMALL[sq]);
    }
    print!("\n\n");
}

pub fn vis_stb() {
    //visualization of the 8x8 -> 12x10 conversion table
    for _sq in 0..64 {
        if _sq % 8 == 0 {
            println!("");
        }
        print!("{} ", SMALL_TO_BIG[_sq]);
    }
    print!("\n\n");
}

pub fn vis_ff120() {
    //visualize FILES_FROM_120
    for sq in 0..120 {
        if sq % 10 == 0 && sq != 0 {print!("\n");}
        print!("{} ", FILES_FROM_120[sq]);
    }
    print!("\n\n");
}

pub fn vis_rf120() {
    //vilualize RANKS_FROM_120
    for sq in 0..120 {
        if sq % 10 == 0 && sq != 0 {print!("\n");}
        print!("{} ", RANKS_FROM_120[sq]);
    }
    print!("\n\n");
}

pub fn vis_pawn_bbs(board: &BoardStructure) {
    //visualize the pawn boards for all three colors
    for color in 0..3 {
        if color == 0 {
            print!("White:\n");
        } else if color == 1 {
            print!("Black:\n");
        } else {
            print!("Both:\n");
        }
        vis_bb(board.pawns[color]);
    }
}

/////////////////////////////////////////////////////////////////////

pub fn vis_bb(bb: u64) {
    //visualize the given bitboard
    let mut sq: usize;
    let mut _sq: usize;

    let tbshifted: u64 = 1u64;

    for rank in (0..8).rev() { //.rev() to have normal board view
        for file in 0..8 { //no .rev() here!
            sq  = (21 + file) + (rank * 10);
            _sq = BIG_TO_SMALL[sq];
            if (lsh(tbshifted, _sq) & bb) != 0u64 {
                print!("F ");
            } else {
                print!("- ");
            }
        }
        print!("\n");
    }
    print!("\n");
}

/////////////////////////////////////////////////////////////////////


pub fn vis_board(board: &BoardStructure) {
    //print the given board
    let mut sq: usize;
    let mut piece: Pieces;

    println!("");

    print!("    ");
    for file in 0..8 {
        print!("{} ", FILE_CHARS[file])
    }

    print!("\n    ");
    for _ in 0..7 {
        print!("__");
    }
    print!("_");

    println!("");

    for rank in (0..8).rev() {
        print!("{}  |", rank + 1);
        for file in 0..8 {
            sq = (21 + file) + (rank * 10);
            piece = board.board[sq];
            print!("{} ", PIECE_CHARS[piece as usize]);
        }
        print!("\n");
    }
    
    println!("");
    print!("side to move: {}", PLAYER_CHARS[board.side as usize]);
    print!("\nen passent on: ");
    vis_sq(board.en_pas);
    print!("\ncastle permissions: {}{}{}{}",
            if (board.castle_perm & CastlingRights::WkR as u8) != 0 {'K'} else {'-'},
            if (board.castle_perm & CastlingRights::WqR as u8) != 0 {'Q'} else {'-'},
            if (board.castle_perm & CastlingRights::BkR as u8) != 0 {'k'} else {'-'},
            if (board.castle_perm & CastlingRights::BqR as u8) != 0 {'q'} else {'-'},
            );
    print!("\nboard key: {}", board.pos_key);
    print!("\npoly key: {:x?}", polykey_from_board(board));
    print!("\n\n");
}

pub fn vis_attsqs(side: Players, board: &BoardStructure) {
    //shows all squares which the given side attacks in the current board
    let mut sq: usize;

    for rank in (0..8).rev() {
        for file in 0..8 {
            sq = (21 + file) + (rank * 10);
            if is_attacked(sq, side, board) {
                print!("A ");
            } else {
                print!("- ");
            }
        }
        print!("\n");
    }
    print!("\n");
}



pub fn vis_sq(sq: usize) {
    //prints the given sqaure in algebraic notation

    if sq == 0 {
        print!("-");
    } else {

        let file: u8= FILES_FROM_120[sq] as u8;
        let rank: u8 = RANKS_FROM_120[sq] as u8;

        print!("{}{}", (('a' as u8) + file) as char,
                        (('1' as u8) + rank) as char
                );
    }
}

pub fn vis_mv(action: usize) {
    //prints the given move in algebraic notation

    let prom: usize = promoted(action);

    if prom != 0 {
        let mut prom_char: char = 'q';

        if IS_KING_MASK[prom] {
            prom_char = 'k';
        } else if !IS_BISHOP_QUEEN_MASK[prom] && IS_ROOK_QUEEN_MASK[prom] {
            prom_char = 'r';
        } else if IS_BISHOP_QUEEN_MASK[prom] && !IS_ROOK_QUEEN_MASK[prom] {
            prom_char = 'b';
        } else if IS_KNIGHT_MASK[prom] {
            prom_char = 'n';
        }
        
        vis_sq(from_sq(action));
        vis_sq(to_sq(action));
        print!("{}", prom_char)

    } else {

        vis_sq(from_sq(action));
        vis_sq(to_sq(action));
    }
}


pub fn vis_mvlist(list: &MoveListStructure) {
    //visualize the given move list
    print!("Move list:\n");
    let mut action: usize;
    let mut score: usize;

    for ind in 0..list.length {
        action = list.moves[ind].action;
        score  = list.moves[ind].score;
        print!("Move with score {}: ", score);
        vis_mv(action); 
    }
    print!("Total number of moves: {}", list.length);
}