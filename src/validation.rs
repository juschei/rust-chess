/////////////////////////////////////////////////////////////////////

//internal imports:
use crate::statics::*;
use crate::structs::*;
use crate::hash::*;

//external imports:
use std::convert::TryInto;

/////////////////////////////////////////////////////////////////////

pub fn sq_on_board(sq: usize) -> bool {
    if FILES_FROM_120[sq] == Files::NoF as usize || RANKS_FROM_120[sq] == Ranks::NoR as usize {
        return false;
    } else {
        return true;
    }
}

pub fn side_valid(side: usize) -> bool {
    return side == Players::White as usize || side == Players::Black as usize;
}

pub fn piece_valid_or_empty(piece: usize) -> bool {
    return Pieces::Empty as usize <= piece && piece <= Pieces::Bk as usize;
}

pub fn piece_valid(piece: usize) -> bool {
    return Pieces::Wp as usize <= piece && piece <= Pieces::Bk as usize;
}



pub fn check_board(board: &BoardStructure) -> bool {
    //assert if the given board seems valid

    let mut sq: usize;
    let mut temp_piece: usize;
    let mut temp_color: usize;

    let mut temp_piece_nums: [usize; 13] = [0; 13];
    let mut temp_big_pieces: [usize; 2] = [0; 2];
    let mut temp_min_pieces: [usize; 2] = [0; 2];
    let mut temp_maj_pieces: [usize; 2] = [0; 2];
    let mut temp_material: [usize; 2] = [0; 2];

    let mut temp_pawns: [u64; 3] = [0; 3];

    temp_pawns[0] = board.pawns[0];
    temp_pawns[1] = board.pawns[1];
    temp_pawns[2] = board.pawns[2];

    

    for piece in 1..13 {
        for temp_piece_num in 0..board.num_pieces[piece] {
            sq = board.piece_list[piece][temp_piece_num];
            assert!(board.board[sq] as usize == piece);
        }
    }



    for _sq in 0..64 {
        sq = SMALL_TO_BIG[_sq];
        temp_piece = board.board[sq] as usize;
        temp_piece_nums[temp_piece] += 1;
        temp_color = COLOR_MASK[temp_piece] as usize;

        if BIG_MASK[temp_piece] {temp_big_pieces[temp_color] += 1;}
        if MIN_MASK[temp_piece] {temp_min_pieces[temp_color] += 1;}
        if MAJ_MASK[temp_piece] {temp_maj_pieces[temp_color] += 1;}

        if temp_color != 2 {temp_material[temp_color] += VALUES[temp_piece]};
    }


    for temp_piece in 1..13 {
        assert!(temp_piece_nums[temp_piece] == board.num_pieces[temp_piece]);
    }


    assert!(board.num_pieces[Pieces::Wp as usize] == temp_pawns[0].count_ones().try_into().unwrap());
    assert!(board.num_pieces[Pieces::Bp as usize] == temp_pawns[1].count_ones().try_into().unwrap());
    assert!((board.num_pieces[Pieces::Wp as usize] + board.num_pieces[Pieces::Bp as usize]) == temp_pawns[2].count_ones().try_into().unwrap());


    //check bitboard squares with pop function?


    assert!(temp_material[0] == board.mat_strength[0].try_into().unwrap() && temp_material[1] == board.mat_strength[1].try_into().unwrap());
    assert!(temp_min_pieces[0] == board.min_pieces[0].try_into().unwrap() && temp_min_pieces[1] == board.min_pieces[1].try_into().unwrap());
    assert!(temp_maj_pieces[0] == board.maj_pieces[0].try_into().unwrap() && temp_maj_pieces[1] == board.maj_pieces[1].try_into().unwrap());
    assert!(temp_big_pieces[0] == board.big_pieces[0].try_into().unwrap() && temp_big_pieces[1] == board.big_pieces[1].try_into().unwrap());


    assert!(board.side == Players::White || board.side == Players::Black);
    assert!(generate_position_hash(board) == board.pos_key);

    assert!(board.en_pas == Squares::NoSq as usize
            || (RANKS_FROM_120[board.en_pas] == Ranks::R6 as usize && board.side == Players::White)
            || (RANKS_FROM_120[board.en_pas] == Ranks::R3 as usize && board.side == Players::Black));


    assert!(board.board[board.king_sqs[0]] == Pieces::Wk);
    assert!(board.board[board.king_sqs[1]] == Pieces::Bk);

    return false;

}

