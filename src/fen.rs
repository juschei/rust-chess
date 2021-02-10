/////////////////////////////////////////////////////////////////////

//internal imports:
use crate::statics::*;
use crate::bitboards::*;
use crate::structs::*;
use crate::hash::*;

//external imports:
use std::char;

/////////////////////////////////////////////////////////////////////

//clears a board
pub fn reset_board(board: &mut BoardStructure) {
    for sq in 0..120 {
        board.board[sq] = Pieces::OffLimits;
    }

    for _sq in 0..64 {
        board.board[SMALL_TO_BIG[_sq]] = Pieces::Empty;
    }

    for color in 0..2 {
        //initially no pieces on board
        board.big_pieces[color] = 0;
        board.min_pieces[color] = 0;
        board.maj_pieces[color] = 0;

        board.mat_strength[color] = 0;

        //pawn bitboards identicially zero
        board.pawns[color] = 0u64;
    }

    for piece_type in 0..13 {
        board.num_pieces[piece_type] = 0;
    }

    board.king_sqs    = [Squares::NoSq as usize; 2];
    board.side        = Players::Both;
    board.fifty_cnt   = 0;
    board.ply_depth   = 0;
    board.ply         = 0;
    board.castle_perm = 0;
    board.en_pas      = 0;
    board.pos_key     = 0u64;
}


pub fn update_material_lists(board: &mut BoardStructure) {
    let mut piece: usize;
    let mut color: usize;

    for sq in 0..120 {
        piece = board.board[sq] as usize;
        if piece != (Pieces::Empty as usize) && piece != (Pieces::OffLimits as usize) {
            color = COLOR_MASK[piece] as usize;

            //increment the number of big, minor and major pieces
            if BIG_MASK[piece] {board.big_pieces[color] += 1;}
            if MIN_MASK[piece] {board.min_pieces[color] += 1;}
            if MAJ_MASK[piece] {board.maj_pieces[color] += 1;}

            //add value of piece to material
            board.mat_strength[color] += VALUES[piece];

            //set board of piece in piece list
            board.piece_list[piece][board.num_pieces[piece]] = sq;

            //increment number of pieces counter of current piece type
            board.num_pieces[piece] += 1;

            //if piece is king update king squares
            if piece == (Pieces::Wk as usize) {board.king_sqs[color] = sq;}
            if piece == (Pieces::Bk as usize) {board.king_sqs[color] = sq;}

            //if piece if pawn fill up pawn boards
            if piece == (Pieces::Wp as usize) {
                set_bit(&mut board.pawns[0], BIG_TO_SMALL[sq]); //0 is Players::White
                set_bit(&mut board.pawns[2], BIG_TO_SMALL[sq]); //2 is Players::Both
            }
            if piece == (Pieces::Bp as usize) {
                set_bit(&mut board.pawns[1], BIG_TO_SMALL[sq]); //1 is Players::Black
                set_bit(&mut board.pawns[2], BIG_TO_SMALL[sq]); //2 is Players::Both
            }
        }
    }
}

pub fn build_fen(fen: String, board: &mut BoardStructure) {
    //takes an empty board and a fen strings and sets up the corresponding board

    //check that given fen is not empty 
    // assert!(!fen.is_empty());

    //reset board
    reset_board(board);

    // start at rank8, fileA
    let mut rank: usize = 7;
    let mut file: usize = 0;

    let mut empty_count: u8;
    let mut piece: Pieces = Pieces::Empty;
    
    //get mutable copy and reverse
    let mut mut_fen: String = fen;
    mut_fen = mut_fen.chars().rev().collect::<String>();

    //loop through first block of characters and pop
    let mut new_first: char;
    loop {
        //set to 1
        empty_count = 1;

        //pop first character
        new_first = mut_fen.pop().unwrap();
        //match first character
        match new_first {

            //when "/" reset file and decrement rank and continue loop
            '/' => {
                rank -= 1;
                file  = 0;
                continue;
            },

            //when number get number
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8' => {
                piece = Pieces::Empty;
                empty_count = (new_first as u8) - ('0' as u8);
            },

            //check piece type
            'r' => piece = Pieces::Br,
            'n' => piece = Pieces::Bn,
            'b' => piece = Pieces::Bb,
            'q' => piece = Pieces::Bq,
            'k' => piece = Pieces::Bk,
            'p' => piece = Pieces::Bp,

            'R' => piece = Pieces::Wr,
            'N' => piece = Pieces::Wn,
            'B' => piece = Pieces::Wb,
            'Q' => piece = Pieces::Wq,
            'K' => piece = Pieces::Wk,
            'P' => piece = Pieces::Wp,

            //when " " appears in FEN string break while loop
            ' ' => break,

            //
            _   => (),
        }

        //place pieces on board
            //-> Empty squares already covered by reset_board
        let mut _sq: usize;
        for _ in 0..empty_count {
            _sq = file + rank * 8;
            if piece != Pieces::Empty {
                board.board[SMALL_TO_BIG[_sq]] = piece;
            }
            file += 1;
        }
    }
    
    new_first = mut_fen.pop().unwrap();

    //check that fen string specifies a player
    // assert!(new_first == 'w' || new_first == 'b');

    //set current player
    if new_first == 'w' {
        board.side = Players::White;
    } else {
        board.side = Players::Black;
    }

    //pop white space
    mut_fen.pop().unwrap();

    //set castling permission
    for _ in 0..5 {

        new_first = mut_fen.pop().unwrap();

        if new_first == ' ' {
            break
        }
        //Bitwise or with castling rights if there are any
        match new_first {
            'K' => board.castle_perm |= CastlingRights::WkR as u8,
            'Q' => board.castle_perm |= CastlingRights::WqR as u8,
            'k' => board.castle_perm |= CastlingRights::BkR as u8,
            'q' => board.castle_perm |= CastlingRights::BqR as u8,
            
            //complete match
            _   => continue,
        }
    }

    //check if castle permissions were set in bounds
    // assert!(board.castle_perm < 16);

    //check that player is followed by space in fen string
    // assert!(new_first == ' ');


    //check for en passent square
    new_first = mut_fen.pop().unwrap();
    
    if new_first != '-' {
        //get file and rank of en passent square
        file = new_first as usize - 'a' as usize;
        rank = mut_fen.pop().unwrap() as usize - '1' as usize;

        // assert!(file < 8);
        // assert!(rank < 8);

        //set en passent square
        board.en_pas = (21 + file) + (rank * 10)
        };

    //generate board hash key
    board.pos_key = generate_position_hash(board);

    //update martial lists
    update_material_lists(board);
        
}