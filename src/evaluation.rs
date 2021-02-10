/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::structs::*;
use crate::statics::*;
use crate::validation::*;

//external modules:
use rayon::prelude::*;

/////////////////////////////////////////////////////////////////////


pub fn eval(board: &BoardStructure) -> i32 {
    //evaluate the given position 
    let mut score: i32 = board.mat_strength[0] as i32 - board.mat_strength[1] as i32;

//     score += WHITE_USIZES.iter()
//                          .map(|piece_u| {
//                             let mut score: i32 = 0;
//                             let mut _sq: usize;
//                             for piece_num in 0..board.num_pieces[*piece_u] {
//                                 _sq = BIG_TO_SMALL[board.piece_list[*piece_u][piece_num]];
//                                 match piece_u {
//                                     1 => score += PAWN_POS_VALS[_sq],
//                                     2 => score += KNIGHT_POS_VALS[_sq],
//                                     3 => score += BISHOP_POS_VALS[_sq],
//                                     4 => score += ROOK_POS_VALS[_sq],
//                                     5 => score += QUEEN_POS_VALS[_sq],
//                                     6 => score += KING_POS_VALS[_sq],
//                                     _ => panic!("Invalid piece in evaluation")
//                                 };
//                             }
//                             return score;
//                          })
//                          .sum::<i32>();

//    score -= BLACK_USIZES.iter()
//                      .map(|piece_u| {
//                         let mut score: i32 = 0;
//                         let mut mir_sq: usize;
//                         for piece_num in 0..board.num_pieces[*piece_u] {
//                             mir_sq = MIRROR[BIG_TO_SMALL[board.piece_list[*piece_u][piece_num]]];
//                             match piece_u {
//                                 7  => score += PAWN_POS_VALS[mir_sq],
//                                 8  => score += KNIGHT_POS_VALS[mir_sq],
//                                 9  => score += BISHOP_POS_VALS[mir_sq],
//                                 10 => score += ROOK_POS_VALS[mir_sq],
//                                 11 => score += QUEEN_POS_VALS[mir_sq],
//                                 12 => score += KING_POS_VALS[mir_sq],
//                                 _ => panic!("Invalid piece in evaluation")
//                             };
//                         }
//                         return score;
//                      })
//                      .sum::<i32>();      


    let mut piece_u: usize;
    let mut sq: usize;

    if board.ply_depth >= 60 || (board.num_pieces[5 /*Wq*/] == 0 && board.num_pieces[11/*Bq*/] == 0) {
        // ENDGAME

        //pawns
        piece_u = Pieces::Wp as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += PAWN_POS_VALS_ENDGAME[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bp as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score -= PAWN_POS_VALS_ENDGAME[MIRROR[BIG_TO_SMALL[sq]]];
        }
        //knights
        piece_u = Pieces::Wn as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += KNIGHT_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bn as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score -= KNIGHT_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }
        //bishops
        piece_u = Pieces::Wb as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += BISHOP_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bb as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score -= BISHOP_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }
        //rooks
        piece_u = Pieces::Wr as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += ROOK_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Br as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));
            
            score -= ROOK_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }

        //queens
        piece_u = Pieces::Wq as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += QUEEN_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bq as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));
            
            score -= QUEEN_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }

        //kings
        piece_u = Pieces::Wk as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += KING_POS_VALS_ENDGAME[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bk as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));
            
            score -= KING_POS_VALS_ENDGAME[MIRROR[BIG_TO_SMALL[sq]]];
        }
    } else {
        //pawns
        piece_u = Pieces::Wp as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += PAWN_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bp as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score -= PAWN_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }
        //knights
        piece_u = Pieces::Wn as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += KNIGHT_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bn as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score -= KNIGHT_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }
        //bishops
        piece_u = Pieces::Wb as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += BISHOP_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bb as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score -= BISHOP_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }
        //rooks
        piece_u = Pieces::Wr as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += ROOK_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Br as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));
            
            score -= ROOK_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }

        //queens
        piece_u = Pieces::Wq as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += QUEEN_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bq as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));
            
            score -= QUEEN_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }

        //kings
        piece_u = Pieces::Wk as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));

            score += KING_POS_VALS[BIG_TO_SMALL[sq]];
        }
        piece_u = Pieces::Bk as usize;
        for piece_num in 0..board.num_pieces[piece_u] {
            sq = board.piece_list[piece_u][piece_num];

            // assert!(sq_on_board(sq));
            
            score -= KING_POS_VALS[MIRROR[BIG_TO_SMALL[sq]]];
        }
    }

    score = if board.side == Players::White {score} else {-score};

    return score;
}



    