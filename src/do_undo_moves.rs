/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::statics::*;
use crate::structs::*;
use crate::bitboards::*;
use crate::legal_move_gen::*;

//external modules:

/////////////////////////////////////////////////////////////////////

//hashing related

pub fn hash_piece(piece: usize, sq: usize, board: &mut BoardStructure) {
    //hashes piece, square pair key into board key
    board.pos_key ^= PIECE_KEYS[sq][piece];
}
pub fn hash_cast(board: &mut BoardStructure) {
    //hashes castling key in and out of board key
    board.pos_key ^= CASTLING_KEYS[board.castle_perm as usize];
}
pub fn hash_side(board: &mut BoardStructure) {
    //hashes side key in and out of board key
    board.pos_key ^= *SIDE_KEY;
}
pub fn hash_ep(board: &mut BoardStructure) {
    //hashes en passant square key into an out of board key
    board.pos_key ^= PIECE_KEYS[board.en_pas][0] // 0 is Pieces::Empty
}

/////////////////////////////////////////////////////////////////////

pub fn clear_piece(sq: usize, board: &mut BoardStructure) {
    //remove the piece at the given square from the board
    // assert!(board.board[sq] != Pieces::OffLimits && board.board[sq] != Pieces::Empty);

    let piece: Pieces = board.board[sq];
    let piece_u: usize = piece as usize;

    let color: Players = COLOR_MASK[piece_u];
    let color_u: usize = color as usize;

    //update board key
    hash_piece(piece_u, sq, board);

    //update board and material
    board.board[sq]= Pieces::Empty;
    board.mat_strength[color_u] -= VALUES[piece_u];

    if BIG_MASK[piece_u] {
        board.big_pieces[color_u] -= 1;
        if MIN_MASK[piece_u] {
            board.min_pieces[color_u] -= 1;
        } else {
            board.maj_pieces[color_u] -= 1;
        }
    } else {
        clear_bit(&mut board.pawns[color_u], BIG_TO_SMALL[sq]);
        clear_bit(&mut board.pawns[2], BIG_TO_SMALL[sq]);
    }

    //get index in num_pieces list of piece to clear
    // let mut assert: usize = 0;
    let mut clear_index: usize = 0;
    for piece_num in 0..board.num_pieces[piece_u] {
        if board.piece_list[piece_u][piece_num] == sq {
            clear_index = piece_num;
            // assert += 1;
            break;
        }
    } 

    // // assert!(assert != 0);

    //decrement the number of pieces of that type
    board.num_pieces[piece_u] -= 1;

    //compress piece list   
    board.piece_list[piece_u][clear_index] = board.piece_list[piece_u][board.num_pieces[piece_u]];

}

pub fn add_piece(piece: Pieces, sq: usize, board: &mut BoardStructure) {

    let piece_u: usize = piece as usize;

    let color: Players = COLOR_MASK[piece_u];
    let color_u: usize = color as usize;

    //update board key
    hash_piece(piece_u, sq, board);

    //update board and material
    board.board[sq] = piece;
    board.mat_strength[color_u] += VALUES[piece_u];

    if BIG_MASK[piece_u] {
        board.big_pieces[color_u] += 1;
        if MIN_MASK[piece_u] {
            board.min_pieces[color_u] += 1;
        } else {
            board.maj_pieces[color_u] += 1;
        }
    } else {
        set_bit(&mut board.pawns[color_u], BIG_TO_SMALL[sq]);
        set_bit(&mut board.pawns[2], BIG_TO_SMALL[sq])
    }

    //update piece list and number of pieces
    board.piece_list[piece_u][board.num_pieces[piece_u]] = sq; 
    board.num_pieces[piece_u] += 1;
}

pub fn move_piece(from: usize, to: usize, board: &mut BoardStructure) {
    
    // assert!(board.board[from] != Pieces::OffLimits);
    // assert!(board.board[to] != Pieces::OffLimits);

    let piece: Pieces = board.board[from];
    let piece_u: usize = piece as usize;
    
    let color: Players = COLOR_MASK[piece_u];
    let color_u: usize = color as usize;

    //hash piece away from current board and into new square
    hash_piece(piece_u, from, board);
    board.board[from] = Pieces::Empty;
    hash_piece(piece_u, to, board);
    board.board[to] = piece;

    if !BIG_MASK[piece_u] {
        clear_bit(&mut board.pawns[color_u], BIG_TO_SMALL[from]);
        clear_bit(&mut board.pawns[2], BIG_TO_SMALL[from]);
        set_bit(&mut board.pawns[color_u], BIG_TO_SMALL[to]);
        set_bit(&mut board.pawns[2], BIG_TO_SMALL[to]);
    }

    for piece_num in 0..board.num_pieces[piece_u] {
        if board.piece_list[piece_u][piece_num] == from {
            board.piece_list[piece_u][piece_num] = to;
            break;
        }
    }
}

pub fn make_move(action: usize, board: &mut BoardStructure) -> bool {
    //make a move on the board

    let side: Players = board.side;

    let from: usize = from_sq(action);
    let to: usize = to_sq(action);

    // assert!(sq_on_board(from));
    // assert!(sq_on_board(to));
    // assert!(side_valid(side as usize));
    // assert!(piece_valid(board.board[from] as usize));

    board.history[board.ply_depth].pos_key = board.pos_key;

    //check if move is en passant take or castling
    if (action & EP_FLAG) != 0 {
        if side == Players::White {
            clear_piece(to - 10, board);
        } else {
            clear_piece(to + 10, board);
        }  
    } else if (action & CAST_FLAG) != 0 {
        //TODO: IMPROVE MATCHING
        match to {
            23 => move_piece(Squares::A1 as usize, Squares::D1 as usize, board), //C1
            93 => move_piece(Squares::A8 as usize, Squares::D8 as usize, board), //C8
            27 => move_piece(Squares::H1 as usize, Squares::F1 as usize, board), //G1
            97 => move_piece(Squares::H8 as usize, Squares::F8 as usize, board), //G8
            _  => panic!(""),
        }
    }


    //hash out en passant square and castling permissions
    if board.en_pas != (Squares::NoSq as usize) {
        hash_ep(board);
    }
    hash_cast(board);

    //copy current state into history
    board.history[board.ply_depth].action      = action;
    board.history[board.ply_depth].fifty_cnt   = board.fifty_cnt;
    board.history[board.ply_depth].en_pas      = board.en_pas;
    board.history[board.ply_depth].castle_perm = board.castle_perm;

    //update castle permission and en pasant sqaure
    board.castle_perm &= CASTLE_PERM[from];
    board.castle_perm &= CASTLE_PERM[to];
    board.en_pas = Squares::NoSq as usize;

    //hash updated castling permissions back in
    hash_cast(board);


    //update counter for 50 move rule
    let captured: usize = taken(action);
    board.fifty_cnt += 1;
    if captured != Pieces::Empty as usize {
        // assert!(piece_valid(captured));
        clear_piece(to, board);
        //capturing resets 50 move counter
        board.fifty_cnt = 0;
    }



    
    //update ply counters
    board.ply_depth += 1;
    board.ply += 1;


    //add en passant square
    if IS_PAWN_MASK[board.board[from] as usize] {
        //pawn move resets 50 move counter
        board.fifty_cnt = 0;

        //check for x2
        if (action & X2_FLAG) != 0 {
            if side == Players::White {
                board.en_pas = from + 10;
                // assert!(RANKS_FROM_120[board.en_pas] == Ranks::R3 as usize);
            } else {
                board.en_pas = from - 10;
                // assert!(RANKS_FROM_120[board.en_pas] == Ranks::R6 as usize);
            }

            //hash en passant square back in
            hash_ep(board);
        }
    }
    

    //move the piece
    move_piece(from, to, board);


    //handle promotion
    let promoted_to: usize = promoted(action);
    if promoted_to != Pieces::Empty as usize {
        // assert!(!IS_PAWN_MASK[promoted_to] && piece_valid(promoted_to));
        clear_piece(to, board);
        //TODO: DO ALL AS USIZE?
        add_piece(num::FromPrimitive::from_usize(promoted_to).unwrap(), to, board);
    }

    //set king square
    if IS_KING_MASK[board.board[to] as usize] {
        board.king_sqs[side as usize] = to;
    }


    //change side and hash into board key
    board.side = if side == Players::White {Players::Black} else {Players::White};
    hash_side(board);

    //assert!(check_board(board));

    if is_attacked(board.king_sqs[side as usize], board.side, board) {
        //king after move in check: undo move
        unmake_move(board);

        return false;
    }

    return true;
}



pub fn unmake_move(board: &mut BoardStructure) {
    //undo the previous made move

    //assert!(check_board(board));

    board.ply_depth -= 1;
    board.ply -= 1;

    let action: usize = board.history[board.ply_depth].action;
    let from: usize = from_sq(action);
    let to: usize = to_sq(action);

    if board.en_pas != Squares::NoSq as usize {
        hash_ep(board);
    }
    hash_cast(board);

    board.castle_perm = board.history[board.ply_depth].castle_perm;
    board.fifty_cnt = board.history[board.ply_depth].fifty_cnt;
    board.en_pas = board.history[board.ply_depth].en_pas;

    if board.en_pas != Squares::NoSq as usize {
        hash_ep(board);
    }
    hash_cast(board);

    board.side = if board.side == Players::White {Players::Black} else {Players::White};
    hash_side(board);

    if (action & EP_FLAG) != 0 {
        if board.side == Players::White {
            add_piece(Pieces::Bp, to - 10, board);
        } else {
            add_piece(Pieces::Wp, to + 10, board);
        }
    } else if (action & CAST_FLAG) != 0 {
        //TODO: IMPROVE MATCHING
        match to {
            23 => move_piece(Squares::D1 as usize, Squares::A1 as usize, board), //C1
            93 => move_piece(Squares::D8 as usize, Squares::A8 as usize, board), //C8
            27 => move_piece(Squares::F1 as usize, Squares::H1 as usize, board), //G1
            97 => move_piece(Squares::F8 as usize, Squares::H8 as usize, board), //G8
            _  => panic!(""),
        }
    }

    move_piece(to, from, board);

    if IS_KING_MASK[board.board[from] as usize] {
        board.king_sqs[board.side as usize] = from;
    }

    let captured: usize = taken(action);
    if captured != Pieces::Empty as usize {
        //TODO: DO ALL AS USIZE?
        // assert!(piece_valid(captured));
        add_piece(num::FromPrimitive::from_usize(captured).unwrap(), to, board);
    }

    if promoted(action) != Pieces::Empty as usize {
        clear_piece(from, board);
        if COLOR_MASK[promoted(action)] == Players::White {
            add_piece(Pieces::Wp, from, board);
        } else {
            add_piece(Pieces::Bp, from, board);
        }
    }

    //asert!(check_board(board));
}