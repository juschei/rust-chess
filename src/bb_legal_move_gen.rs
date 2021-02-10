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

pub fn legal_move_gen(position: &mut BoardStructurem list: &mut MoveListStructure) {
    //for each piece type build a bitboard


    //calculate attacked squares on squares around king after king is removed
    let mut attacked_sqs: u64 = 0u64;
    let mut sq: usize;
    let mut new_sq: usize;
    let mut piece: Pieces;
    let side: Players = position.side;
    let op_side: Players = if side == Players::White {Players::Black} else {Players::White};

    position.board[position.king_sqs[side as usize]] = Pieces::Empty;

    sq = position.king_sqs[side as usize];
    for direc in KING_DIRECS.iter() {
        new_sq = direc_add(sq, *direc);
        if position.board[sq] != Pieces::OffLimits {
            if is_attacked(sq, op_side, position) {
                attacked_sqs |= 1u64 << BIG_TO_SMALL[sq];
            }
        }
    }

    //add king back to board
    if side == Players::White {
        position.board[position.king_sqs[side as usize]] = Pieces::Wk;
    } else {
        position.board[position.king_sqs[side as usize]] = Pieces::Bk;
    }

    //calculate the number of attacks on the king square
    let mut checkers: u64 = 0u64;
    //knights
    for direc in KNIGHT_DIRECS.iter() {
        new_sq = direc_add(sq, *direc);
        piece = position.board[new_sq];
        if piece != Pieces::OffLimits && COLOR_MASK[piece as usize] != side {
            checkers |= 1u64 << BIG_TO_SMALL[new_sq];
        }
    }

    //bishop and (diagonal) queen
    for direc in BISHOP_DIRECS.iter() {
        new_sq = sq;
        for _ in 0..7 {
            //go to the next square on ray
            new_sq = direc_add(new_sq, *direc);
            //get piece on square
            piece = position.board[new_sq];
            if piece != Pieces::OffLimits {
                //if not OffLimits: check for opposing color
                if COLOR_MASK[piece as usize] == op_side {
                    //iff oppsoing piece: mark checker and break
                    checkers |= 1u64 << BIG_TO_SMALL[new_sq];
                    break;
                } else if piece != Pieces::Empty {
                    //if square non empty and not of opposing color: break
                    break;
                }
            } else {
                //if OffLimits: break ray
                break;
            }
        }
    }

    //rook and (straight) queen
    //bishop and (diagonal) queen
    for direc in ROOK_DIRECS.iter() {
        new_sq = sq;
        for _ in 0..7 {
            //go to the next square on ray
            new_sq = direc_add(new_sq, *direc);
            //get piece on square
            piece = position.board[new_sq];
            if piece != Pieces::OffLimits {
                //if not OffLimits: check for opposing color
                if COLOR_MASK[piece as usize] == op_side {
                    //iff oppsoing piece: mark checker and break
                    checkers |= 1u64 << BIG_TO_SMALL[new_sq];
                    break;
                } else if piece != Pieces::Empty {
                    //if square non empty and not of opposing color: break
                    break;
                }
            } else {
                //if OffLimits: break ray
                break;
            }
        }
    }

    let mut capture_mask: u64 = std::u64::MAX;
    let mut push_mask: u64 = std::u64::MAX;
    let num_checks = checkers.count_ones();

    if num_checks > 1 {
        //-> 2 or more checks means that only king can move
    } else if num_checks == 1 {
        //-> 1 check:
            //-> move king
            //-> capture checking piece
            //-> block checking piece (only possible if attacker is b,r,q)
            // (for the last to create push and capture masks)
            //(build attackers bitboard and count the ones, ignore opposing king)
            //-> in check means no castling
        //index of checker is position of the single 1 in "checkers"
        let checker_square = checkers.trailing_zeros();
        if IS_SLIDER[position.board[SMALL_TO_BIG[checker_square]]] {
            //push_mask
        }
    } else {
        //no check
        //calculate pinned pieces
        //calculate possible moves of pinned pieces
        //check for double discovered check
    }
    
}

pub fn slider_rays_to_square(king_sq: usize, checker_square: usize, position: &BoardStructure) {
    //calculates the squares the current player can push his pieces to to block
    //a given check by sliding pieces of the opposition
    let piece: Pieces = position.board[checker_square];

    if position.side == Players::White {
        if piece == Pieces::Bb {

        } else if piece == Pieces::Br {

        } else {
            //Black queen

        }
    } else {
        if piece == Pieces::Wb {

        } else if piece == Pieces::Wr {

        } else {
            //White queen
        }
    }
    
}

pub fn pins_to_sq(sq: uisze, position: &BoardStructure) {

    let mut new_sq: usize = sq;
    let mut own_count: usize;
    let mut piece: Pieces;

    let side: Players = position.side;
    let op_side: Players = if side == Players::White {Players:: Black} else {Players::White};

    //loop through all direction where a pin could be
    for direc in KING_DIRECS.iter() {
        //set number of own pieces on this ray to zero
        own_count = 0;
        //walk through ray
        for d in 0..7 {
            //get new square, piece on square and color of that piece
            new_sq = direc_add(new_sq, *direc);
            piece = position.board[new_sq];
            piece_color = COLOR_MASK[piece as usize];

            if piece_color == op_side {
                //if opposite colored piece:
                if own_count == 0 {
                    //if there are no pieces of current player in between: check
                } else {
                    //piece of current player in between: pin

                }
            } else if piece_color == side {
                //if piece is of current player:
                if own_count < 1 {
                    //if we haven't already found a piece of current player, increment
                    own_count += 1;
                } else {
                    //already two own pieces who block in this direction: no pin
                    break;
                }
            }
        }
    }

    if position.side == Players::White {
        //look at threats from black
        for piece in BLACK_SLIDERS.iter() {

        }
    } else {
        //look at threats from white
        for piece in WHITE_SLIDERS.iter() {

        }
    }