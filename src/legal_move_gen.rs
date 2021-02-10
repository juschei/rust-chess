/////////////////////////////////////////////////////////////////////

//internal imports:
use crate::statics::*;
use crate::structs::*;

//external imports:
use std::convert::TryInto;

/////////////////////////////////////////////////////////////////////

pub fn direc_add(sq: usize, direc: i8) -> usize {
    //takes an usize and an i8, adds them and converts into uize
    return (sq as i8 + direc).try_into().unwrap();
}

pub fn is_attacked(sq: usize, side: Players, board: &BoardStructure) -> bool {
    //checky if the given square is attacked in the current board
    
    // assert!(sq_on_board(sq));
    // assert!(side_valid(side as usize));
    // assert!(check_board(board));

    //check for attacking pawns
    if side == Players::White {
        if board.board[sq - 11] == Pieces::Wp || board.board[sq - 9] == Pieces::Wp {
            return true;
        }
    } else {
        if board.board[sq + 11] == Pieces::Bp || board.board[sq + 9] == Pieces::Bp {
            return true;
        }
    }

    //check for attacking knights
    let mut piece: usize;
    for direc in KNIGHT_DIRECS.iter() {
        piece = board.board[direc_add(sq, *direc)] as usize;
        if piece != (Pieces::OffLimits as usize) {
            if IS_KNIGHT_MASK[piece] && COLOR_MASK[piece] == side {
                return true;
            }
        }
    }

    //check bishops and (diagonal) queens
    let mut new_sq: usize;
    for direc in BISHOP_DIRECS.iter() {
        new_sq = direc_add(sq, *direc);
        piece = board.board[new_sq] as usize;

        while piece != (Pieces::OffLimits as usize) {
            if piece != (Pieces::Empty as usize) {
                if IS_BISHOP_QUEEN_MASK[piece] && COLOR_MASK[piece] == side {
                    return true;
                }
                break;
            }
            new_sq = direc_add(new_sq, *direc);
            piece = board.board[new_sq] as usize;
        }
    }

    //check for rooks and (straight) queens
    for direc in ROOK_DIRECS.iter() {
        new_sq = direc_add(sq, *direc);
        piece = board.board[new_sq] as usize;

        while piece != (Pieces::OffLimits as usize) {
            if piece != (Pieces::Empty as usize) {
                if IS_ROOK_QUEEN_MASK[piece] && COLOR_MASK[piece] == side {
                    return true;
                }
                break;
            }
            new_sq = direc_add(new_sq, *direc);
            piece = board.board[new_sq] as usize;
        }
    }

    //check for kings
    for direc in KING_DIRECS.iter() {
        piece = board.board[direc_add(sq, *direc)] as usize;
        if piece != (Pieces::OffLimits as usize) {
            if IS_KING_MASK[piece] && COLOR_MASK[piece] == side {
                return true;
            }
        }
    }

    //if nothing happened then square is not attacked
    return false;
}


pub fn from_sq(m: usize) -> usize {
    //extract the from square of a move
    return m & 0x7F;
}
pub fn to_sq(m: usize) -> usize {
    //extract the to square of a move
    return (m >> 7) & 0x7F;
}
pub fn taken(m: usize) -> usize {
    //extract the taken piece of a move
    return (m >> 14) & 0xF;
}
pub fn promoted(m: usize) -> usize {
    //extracts the piece to which was promoted in a move (if there is any)
    return (m >> 18) & 0xF;
}


pub fn create_move(f: usize, t: usize ,cap: usize, prom: usize, fl: usize) -> usize {
    //takes a from and to square, the captured piece, the piece to which the moving piece
    //was promoted (if there is any) aswell as a flag that indicates en passent, a pawn
    //start or castling and builds the bit representation of the corresponding move
    return f | t << 7 | cap << 14 | prom << 18 | fl;
}



//COMBINE ALL THREE ADD_XYZ_MOVE FUNCTIONS?
pub fn add_quiet_move(action: usize, board: &BoardStructure, list: &mut MoveListStructure) {
    //add move to list
    list.moves[list.length].action = action;

    if board.search_killers[board.ply][0] == action {
        list.moves[list.length].score = 900000;
    } else if board.search_killers[board.ply][1] == action {
        list.moves[list.length].score = 800000;
    } else {
        list.moves[list.length].score = board.search_history[to_sq(action)][board.board[from_sq(action)] as usize];
    }

    list.length += 1;
}

pub fn add_loud_move(action: usize, board: &BoardStructure, list: &mut MoveListStructure) {
    //add move to list
    list.moves[list.length].action = action;
    list.moves[list.length].score = MVV_LVA[taken(action)][board.board[from_sq(action)] as usize] + 1000000;
    list.length += 1;
}

pub fn add_en_pas_move(action: usize, list: &mut MoveListStructure) {
    //add move to list
    list.moves[list.length].action = action;
    list.moves[list.length].score = 105 + 1000000;
    list.length += 1;
}

//COMBINE ADD_WPAWN_CAP_MOVE AND ADD_WPAWN_NCAP_MOVE?
pub fn add_wpawn_cap_move(board: &BoardStructure, from:usize, to: usize, cap: usize, list: &mut MoveListStructure) {
    if RANKS_FROM_120[from] == (Ranks::R7 as usize) {
        //if pawn moves from rank 7 forward it will be promoted, so add all possible promotes
        add_loud_move(create_move(from, to, cap, Pieces::Wq as usize, 0), board, list);
        add_loud_move(create_move(from, to, cap, Pieces::Wr as usize, 0), board, list);
        add_loud_move(create_move(from, to, cap, Pieces::Wb as usize, 0), board, list);
        add_loud_move(create_move(from, to, cap, Pieces::Wn as usize, 0), board, list);
    } else {
        //if pawn is not on rank 7 and moves it will not be promoted
        add_loud_move(create_move(from, to, cap, Pieces::Empty as usize, 0), board, list);
    }
}

pub fn add_wpawn_ncap_move(board: &BoardStructure, from: usize, to: usize, list: &mut MoveListStructure) {
    if RANKS_FROM_120[from] == (Ranks::R7 as usize) {
        //if pawn moves from rank 7 forward it will be promoted, so add all possible promotes
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Wq as usize, 0), board, list);
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Wr as usize, 0), board, list);
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Wb as usize, 0), board, list);
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Wn as usize, 0), board, list);
    } else {
        //if pawn is not on rank 7 and moves it will not be promoted
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Empty as usize, 0), board, list);
    }
}

//COMBINE ADD_BPAWN_CAP_MOVE AND ADD_BPAWN_NCAP_MOVE?
pub fn add_bpawn_cap_move(board: &BoardStructure, from:usize, to: usize, cap: usize, list: &mut MoveListStructure) {
    if RANKS_FROM_120[from] == (Ranks::R2 as usize) {
        //if pawn moves from rank 2 forward it will be promoted, so add all possible promotes
        add_loud_move(create_move(from, to, cap, Pieces::Bq as usize, 0), board, list);
        add_loud_move(create_move(from, to, cap, Pieces::Br as usize, 0), board, list);
        add_loud_move(create_move(from, to, cap, Pieces::Bb as usize, 0), board, list);
        add_loud_move(create_move(from, to, cap, Pieces::Bn as usize, 0), board, list);
    } else {
        //if pawn is not on rank 2 and moves it will not be promoted
        add_loud_move(create_move(from, to, cap, Pieces::Empty as usize, 0), board, list);
    }
}

pub fn add_bpawn_ncap_move(board: &BoardStructure, from: usize, to: usize, list: &mut MoveListStructure) {
    if RANKS_FROM_120[from] == (Ranks::R2 as usize) {
        //if pawn moves from rank 2 forward it will be promoted, so add all possible promotes
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Bq as usize, 0), board, list);
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Br as usize, 0), board, list);
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Bb as usize, 0), board, list);
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Bn as usize, 0), board, list);
    } else {
        //if pawn is not on rank 2 and moves it will not be promoted
        add_quiet_move(create_move(from, to, Pieces::Empty as usize, Pieces::Empty as usize, 0), board, list);
    }
}

pub fn pseudo_legal_move_gen(board: &BoardStructure, list: &mut MoveListStructure) {
    //for a given board return all legal moves of current player
    let side: Players = board.side;
    let _op_side: Players = if side == Players::White {Players::Black} else {Players::White};
    let mut sq: usize;
    let mut new_sq: usize;
    let mut target_piece: Pieces;

    //pawns
    if side == Players::White {
        //if current player is white, loop through all white pawns
        for piece_num in 0..board.num_pieces[Pieces::Wp as usize] {
            //get board of current pawn
            sq = board.piece_list[Pieces::Wp as usize][piece_num];

            // assert!(board.board[sq] != Pieces::OffLimits);

            if board.board[sq + 10] == Pieces::Empty {
                //if square infront of pawn is Empty add x1
                add_wpawn_ncap_move(board, sq, sq + 10, list);

                if  RANKS_FROM_120[sq] == (Ranks::R2 as usize) && board.board[sq + 20] == Pieces::Empty {
                    //if pawn is one the second file and the square two infront is Empty add x2
                    add_quiet_move(create_move(sq, sq + 20, Pieces::Empty as usize, Pieces::Empty as usize, X2_FLAG), board, list);
                }
            }

            //check if pawn can take diagonally
            let mut piece: Pieces;
            for new_sq in [sq + 9, sq + 11].iter() {
                piece = board.board[*new_sq];
                if piece != Pieces::OffLimits {
                    //if new square is on the board:
                    if COLOR_MASK[board.board[*new_sq] as usize] == Players::Black {
                        //if new square is occupied by black:
                        add_wpawn_cap_move(board, sq, *new_sq, board.board[*new_sq] as usize, list);
                    }
                    if *new_sq == board.en_pas && *new_sq != Squares::NoSq as usize {
                        //if new square is en passant square add en passant capture
                        add_en_pas_move(create_move(sq, *new_sq, Pieces::Empty as usize, Pieces::Empty as usize, EP_FLAG), list);
                    }
                }
            }

        }
    } else {
        //if current player is not White, then it's Black, so loop through all white pawns
        for piece_num in 0..board.num_pieces[Pieces::Bp as usize] {
            //get board of current pawn
            sq = board.piece_list[Pieces::Bp as usize][piece_num];

            // assert!(board.board[sq] != Pieces::OffLimits);

            if board.board[sq - 10] == Pieces::Empty {
                //if square infront of pawn is Empty add x1
                add_bpawn_ncap_move(board, sq, sq - 10, list);

                if  RANKS_FROM_120[sq] == (Ranks::R7 as usize) && board.board[sq - 20] == Pieces::Empty {
                    //if pawn is one the second file and the square two infront is Empty add x2
                    add_quiet_move(create_move(sq, sq - 20, Pieces::Empty as usize, Pieces::Empty as usize, X2_FLAG), board, list);
                }
            }

            //check if pawn can take diagonally
            let mut piece: Pieces;
            for new_sq in [sq - 9, sq - 11].iter() {
                piece = board.board[*new_sq];
                if piece != Pieces::OffLimits {
                    //if new square is on the board:
                    if COLOR_MASK[board.board[*new_sq] as usize] == Players::White {
                        //if new square is occupied by White:
                        add_bpawn_cap_move(board, sq, *new_sq, board.board[*new_sq] as usize, list);
                    }
                    if *new_sq == board.en_pas && *new_sq != Squares::NoSq as usize {
                        //if new square is en passant square add en passant caputre
                        add_en_pas_move(create_move(sq, *new_sq, Pieces::Empty as usize, Pieces::Empty as usize, EP_FLAG), list);
                    }
                }
            }

        }
    }


    //non-sliding pieces -> kings, knights
    
    let type_list: [Pieces; 2] = if side == Players::White {
        [Pieces::Wn, Pieces::Wk]
    } else {
        [Pieces::Bn, Pieces::Bk]
    };

    for piece in type_list.iter() {
        for piece_num in 0..board.num_pieces[*piece as usize] {
            sq = board.piece_list[*piece as usize][piece_num];

            // assert!(board.board[sq] != Pieces::OffLimits);

            //get possible move directions of piece
            let direcs: [i8; 8] = if *piece == Pieces::Wk || *piece == Pieces::Bk {
                KING_DIRECS
            } else {
                KNIGHT_DIRECS
            };

            for direc in direcs.iter() {
                new_sq = direc_add(sq, *direc);

                target_piece = board.board[new_sq];
                if target_piece != Pieces::OffLimits {
                    if target_piece != Pieces::Empty {
                        if COLOR_MASK[target_piece as usize] != side {
                            //add loud move
                            add_loud_move(create_move(sq, new_sq, target_piece as usize, Pieces::Empty as usize, 0), board, list);
                        }
                    } else {
                        //add quiet move
                        add_quiet_move(create_move(sq, new_sq, Pieces::Empty as usize, Pieces::Empty as usize, 0), board, list);
                    }
                }
            }
        }
    }



    //sliding pieces -> bishop, rooks, queens

    let type_list = if side == Players::White {
        [Pieces::Wb, Pieces::Wr, Pieces::Wq]
    } else {
        [Pieces::Bb, Pieces::Br, Pieces::Bq]
    };

    for piece in type_list.iter() {
        for piece_num in 0..board.num_pieces[*piece as usize] {
            sq = board.piece_list[*piece as usize][piece_num];

            // assert!(board.board[sq] != Pieces::OffLimits);

            //get possible move directions of piece
            let direcs = if *piece == Pieces::Wb || *piece == Pieces::Bb {
                BISHOP_DIRECS_FILLED
            } else if *piece == Pieces::Wr || *piece == Pieces::Br {
                ROOK_DIRECS_FILLED
            } else {
                KING_DIRECS
            };

            for direc in direcs.iter() {
                //filter out all directions fills
                if *direc == 0 {
                    continue;
                }
                new_sq = direc_add(sq, *direc);
                target_piece = board.board[new_sq];

                while target_piece != Pieces::OffLimits {
                    
                    if target_piece != Pieces::Empty {
                        if COLOR_MASK[target_piece as usize] != side {
                            //add loud move
                            add_loud_move(create_move(sq, new_sq, target_piece as usize, Pieces::Empty as usize, 0), board, list);
                        }
                        break;

                    } else {
                        //add quiet move
                        add_quiet_move(create_move(sq, new_sq, Pieces::Empty as usize, Pieces::Empty as usize, 0), board, list);
                    }
                    new_sq = direc_add(new_sq, *direc);
                    target_piece = board.board[new_sq];
                }
            }
        }
    }


    //castling
    if side == Players::White {
        if (board.castle_perm & 8u8) != 0 {
            if board.board[Squares::F1 as usize] == Pieces::Empty && board.board[Squares::G1 as usize] == Pieces::Empty {
                if !is_attacked(Squares::E1 as usize, Players::Black, board) && !is_attacked(Squares::F1 as usize, Players::Black, board) {
                    add_quiet_move(create_move(Squares::E1 as usize, Squares::G1 as usize, Pieces::Empty as usize, Pieces::Empty as usize, CAST_FLAG), board, list);
                }
            }
        }
        if (board.castle_perm & 4u8) != 0 {
            if board.board[Squares::D1 as usize] == Pieces::Empty && board.board[Squares::C1 as usize] == Pieces::Empty && board.board[Squares::B1 as usize] == Pieces::Empty {
                if !is_attacked(Squares::E1 as usize, Players::Black, board) && !is_attacked(Squares::D1 as usize, Players::Black, board) {
                    add_quiet_move(create_move(Squares::E1 as usize, Squares::C1 as usize, Pieces::Empty as usize, Pieces::Empty as usize, CAST_FLAG), board, list);
                }
            }
        }
    } else {
        if (board.castle_perm & 2u8) != 0 {
            if board.board[Squares::F8 as usize] == Pieces::Empty && board.board[Squares::G8 as usize] == Pieces::Empty {
                if !is_attacked(Squares::E8 as usize, Players::White, board) && !is_attacked(Squares::F8 as usize, Players::White, board) {
                    add_quiet_move(create_move(Squares::E8 as usize, Squares::G8 as usize, Pieces::Empty as usize, Pieces::Empty as usize, CAST_FLAG), board, list);
                }
            }
        }
        if (board.castle_perm & 1u8) != 0 {
            if board.board[Squares::D8 as usize] == Pieces::Empty && board.board[Squares::C8 as usize] == Pieces::Empty && board.board[Squares::B8 as usize] == Pieces::Empty {
                if !is_attacked(Squares::E8 as usize, Players::White, board) && !is_attacked(Squares::D8 as usize, Players::White, board) {
                    add_quiet_move(create_move(Squares::E8 as usize, Squares::C8 as usize, Pieces::Empty as usize, Pieces::Empty as usize, CAST_FLAG), board, list);
                }
            }
        }
    }
}



pub fn pseudo_legal_captures(board: &BoardStructure, list: &mut MoveListStructure) {
    //for a given board return all legal moves of current player
    let side: Players = board.side;
    let _op_side: Players = if side == Players::White {Players::Black} else {Players::White};
    let mut sq: usize;
    let mut new_sq: usize;
    let mut target_piece: Pieces;

    //pawns
    if side == Players::White {
        //if current player is white, loop through all white pawns
        for piece_num in 0..board.num_pieces[Pieces::Wp as usize] {
            //get board of current pawn
            sq = board.piece_list[Pieces::Wp as usize][piece_num];
            // assert!(board.board[sq] != Pieces::OffLimits);

            //check if pawn can take diagonally
            let mut piece: Pieces;
            for new_sq in [sq + 9, sq + 11].iter() {
                piece = board.board[*new_sq];
                if piece != Pieces::OffLimits {
                    //if new square is on the board:
                    if COLOR_MASK[board.board[*new_sq] as usize] == Players::Black {
                        //if new square is occupied by black:
                        add_wpawn_cap_move(board, sq, *new_sq, board.board[*new_sq] as usize, list);
                    }
                    if *new_sq == board.en_pas && *new_sq != Squares::NoSq as usize {
                        //if new square is en passant square add en passant capture
                        add_en_pas_move(create_move(sq, *new_sq, Pieces::Empty as usize, Pieces::Empty as usize, EP_FLAG), list);
                    }
                }
            }

        }
    } else {
        //if current player is not White, then it's Black, so loop through all white pawns
        for piece_num in 0..board.num_pieces[Pieces::Bp as usize] {
            //get board of current pawn
            sq = board.piece_list[Pieces::Bp as usize][piece_num];
            // assert!(board.board[sq] != Pieces::OffLimits);

            //check if pawn can take diagonally
            let mut piece: Pieces;
            for new_sq in [sq - 9, sq - 11].iter() {
                piece = board.board[*new_sq];
                if piece != Pieces::OffLimits {
                    //if new square is on the board:
                    if COLOR_MASK[board.board[*new_sq] as usize] == Players::White {
                        //if new square is occupied by White:
                        add_bpawn_cap_move(board, sq, *new_sq, board.board[*new_sq] as usize, list);
                    }
                    if *new_sq == board.en_pas && *new_sq != Squares::NoSq as usize {
                        //if new square is en passant square add en passant caputre
                        add_en_pas_move(create_move(sq, *new_sq, Pieces::Empty as usize, Pieces::Empty as usize, EP_FLAG), list);
                    }
                }
            }

        }
    }


    //non-sliding pieces //kings, knights
    
    let type_list: [Pieces; 2] = if side == Players::White {
        [Pieces::Wn, Pieces::Wk]
    } else {
        [Pieces::Bn, Pieces::Bk]
    };

    for piece in type_list.iter() {
        for piece_num in 0..board.num_pieces[*piece as usize] {
            sq = board.piece_list[*piece as usize][piece_num];
            // assert!(board.board[sq] != Pieces::OffLimits);

            //get possible move directions of piece
            let direcs: [i8; 8] = if *piece == Pieces::Wk || *piece == Pieces::Bk {
                KING_DIRECS
            } else {
                KNIGHT_DIRECS
            };

            for direc in direcs.iter() {
                new_sq = direc_add(sq, *direc);

                target_piece = board.board[new_sq];
                if target_piece != Pieces::OffLimits {
                    if target_piece != Pieces::Empty {
                        if COLOR_MASK[target_piece as usize] != side {
                            //add loud move
                            add_loud_move(create_move(sq, new_sq, target_piece as usize, Pieces::Empty as usize, 0), board, list);
                        }
                    }
                }
            }
        }
    }



    //sliding pieces -> bishop, rooks, queens

    let type_list = if side == Players::White {
        [Pieces::Wb, Pieces::Wr, Pieces::Wq]
    } else {
        [Pieces::Bb, Pieces::Br, Pieces::Bq]
    };

    for piece in type_list.iter() {
        for piece_num in 0..board.num_pieces[*piece as usize] {
            sq = board.piece_list[*piece as usize][piece_num];
            // assert!(board.board[sq] != Pieces::OffLimits);

            //get possible move directions of piece
            let direcs = if *piece == Pieces::Wb || *piece == Pieces::Bb {
                BISHOP_DIRECS_FILLED
            } else if *piece == Pieces::Wr || *piece == Pieces::Br {
                ROOK_DIRECS_FILLED
            } else {
                KING_DIRECS
            };

            for direc in direcs.iter() {
                //filter out all directions fills
                if *direc == 0 {
                    continue;
                }
                new_sq = direc_add(sq, *direc);
                target_piece = board.board[new_sq];

                while target_piece != Pieces::OffLimits {
                    
                    if target_piece != Pieces::Empty {
                        if COLOR_MASK[target_piece as usize] != side {
                            //add loud move
                            add_loud_move(create_move(sq, new_sq, target_piece as usize, Pieces::Empty as usize, 0), board, list);
                        }
                        break;

                    } 
                    new_sq = direc_add(new_sq, *direc);
                    target_piece = board.board[new_sq];
                }
            }
        }
    }
}