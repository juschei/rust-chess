/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::structs::*;
use crate::statics::*;
use crate::visualization::*;
use crate::legal_move_gen::*;
use crate::do_undo_moves::*;
use crate::principal_variation::*;
use crate::evaluation::*;
use crate::polyglot::*;
use crate::trans_table::*;

//external modules:

/////////////////////////////////////////////////////////////////////

pub fn get_time() -> u128 {
    let start = std::time::SystemTime::now();
    let since_epoch = start.duration_since(std::time::UNIX_EPOCH).expect("");
    return since_epoch.as_millis();
}

fn check_time(info: &mut SearchInfo) {
    //check is time is up or interrupted
    if info.timed && get_time() > info.stop_time {
        info.stopped = true;
    }
}

fn is_rep(board: &BoardStructure) -> bool {
    //check if the current position is a threefold repetition
    let mut cnt: u8 = 0;

    if board.ply_depth > 0 {
        for ply in (board.ply_depth - board.fifty_cnt)..(board.ply_depth - 1) {
            if board.pos_key == board.history[ply].pos_key {
                cnt += 1;
            }
        }
        if cnt == 3 {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn clear_info(board: &mut BoardStructure, info: &mut SearchInfo) {

    //clear info in board
    board.search_history = [[0usize; 13]; 120];
    board.search_killers = [[0usize; 2]; 64];
    // board.pv_table = Default::default();
    board.ply = 0;

    // info.start_time = get_time(); //useless due to parse go
    info.stopped    = false;
    info.nodes      = 0;

    info.fail_high = 0.0;
    info.fail_high_fst = 0.0;
}


fn next_move_to_search(move_num: usize, list: &mut MoveListStructure) {
    //pick next move to search in alpha beta
    let mut best_score: usize = 0;
    let mut best_move_num = move_num;

    //search for better score
    for index in move_num..list.length {
        if list.moves[index].score > best_score {
            best_score = list.moves[index].score;
            best_move_num = index;
        }
    }

    //swap entries so best move is at place of given
    let copy = list.moves[move_num];
    list.moves[move_num] = list.moves[best_move_num];
    list.moves[best_move_num] = copy;
}


fn alpha_beta(board: &mut BoardStructure, info: &mut SearchInfo, mut alpha: i32, beta: i32, mut depth: usize, config: &ConfigInfo) -> i32 {
    
    if depth == 0 {
        return quiescence(board, alpha, beta, info, config);
    }

    //check for time
    if info.nodes & 4097 == 0 {
        check_time(info)
    }
    
    info.nodes += 1;

    if config.threefold_null {
        if is_rep(board) {
            return 0;
        }
    } else if config.fifty_null {
        if board.fifty_cnt >= 100 {
            return 0;
        }
    }


    //increase depth when in check to mitigate horizon effect for checks
    let op_side: Players = if board.side == Players::White {Players::Black} else {Players::White};
    let king_attacked: bool = is_attacked(board.king_sqs[board.side as usize], op_side, board);

    if king_attacked {
        depth += 1;
    }


    //check for matching transposition table entry
    let mut score: i32 = - INF;
    let mut pv_move: usize = 0;

    if probe_trans_table(board, &mut pv_move, &mut score, alpha, beta, depth) {
        return score;
    }


    let mut move_list: MoveListStructure = Default::default();
    pseudo_legal_move_gen(board, &mut move_list);

    let mut legal_count: usize = 0;

    let old_alpha: i32 = alpha;

    let mut best_action: usize = 0;
    let mut best_score: i32 = - INF;


    //check for pv
    if pv_move != 0 {
        for move_num in 0..move_list.length {
            if move_list.moves[move_num].action == pv_move {
                //score pv move the highest
                move_list.moves[move_num].score = 2000000;
                break;
            }
        }
        
    }


    //loop through all pseudo legal moves of the board
    for move_num in 0..move_list.length {

        //swap current move for move with best score
        next_move_to_search(move_num, &mut move_list);

        //make move and check for validity
        if !make_move(move_list.moves[move_num].action, board) {
            continue;
        }

        legal_count += 1;

        score = - alpha_beta(board, info, -beta, -alpha, depth - 1, config);

        //unmake previously made move
        unmake_move(board);

        //stop if we are out of time
        if info.stopped {
            return 0;
        }

        //check if current score is greater than previously best score
        if score > best_score {

            //remember best score and best action for move ordering
            best_score = score;
            best_action = move_list.moves[move_num].action;

            //check for cutoffs
            if score > alpha {
                //alpha cutoff
                if score >= beta{
                    //beta cutoff

                    if config.fhfquot {
                        //count ratio of fail highs in the first checked move
                        if legal_count == 1 {
                            info.fail_high_fst += 1.0; //was fail high on first move?
                        }
                        info.fail_high += 1.0; //count fail highs
                    }


                    //killer heuristic
                    if move_list.moves[move_num].action & CAP_FLAG == 0 {
                        board.search_killers[board.ply][1] = board.search_killers[board.ply][0];
                        board.search_killers[board.ply][0] = move_list.moves[move_num].action;
                    }

                    store_trans_move(board, best_action, beta, depth, TransFlag::Beta);

                    return beta;
                }
                
                alpha = score;
                best_action = move_list.moves[move_num].action;

                //search history heuristic
                if move_list.moves[move_num].action & CAP_FLAG == 0 {
                    board.search_history[to_sq(best_action)][board.board[from_sq(best_action)] as usize] += depth;
                }
            }
        }
    }

    //check for check- and stalemate
    if legal_count == 0 {
        if king_attacked {
            //checkmate
            return -MATE + (board.ply as i32);
        } else {
            //stalemate
            return 0;
        }
    }

    if alpha != old_alpha {
        store_trans_move(board, best_action, best_score, depth, TransFlag::Exact);
    } else {
        store_trans_move(board, best_action, alpha, depth, TransFlag::Alpha);
    }
    
    return alpha;
}

fn quiescence(board: &mut BoardStructure, mut alpha: i32, beta: i32, info: &mut SearchInfo, config: &ConfigInfo) -> i32 {
    //perform quiescence search on the given board

     //check for time
     if info.nodes & 8192 == 0 {
        check_time(info)
    }

    info.nodes += 1;

    if config.threefold_null {
        if is_rep(board) {
            return 0;
        }
    } else if config.fifty_null {
        if board.fifty_cnt >= 100 {
            return 0;
        }
    } else if board.ply > 63 {
        return eval(board);
    }

    //standing pat
    let mut score: i32 = eval(board);

    if score >= beta {
        return beta;
    } 

    if score > alpha {
        alpha = score;
    }

    let mut move_list: MoveListStructure = Default::default();
    pseudo_legal_captures(board, &mut move_list);

    let mut legal_count: usize = 0;
    // let mut best_action: usize = 0;

    // let old_alpha: i32 = alpha;

    for move_num in 0..move_list.length {

        //pick next move to look at
        next_move_to_search(move_num, &mut move_list);

        //check if move is legal
        if !make_move(move_list.moves[move_num].action, board) {
            continue;
        }

        legal_count += 1;

        score = - quiescence(board, - beta, - alpha, info, config);

        //unmake previous move
        unmake_move(board);

        //stop if out of time
        if info.stopped {
            return 0;
        }

        if score > alpha {
            //alpha cutoof
            if score >= beta {
                //beta cutoff

                if config.fhfquot {
                    if legal_count == 1 {
                        info.fail_high_fst += 1.0;
                    }
                    info.fail_high += 1.0;
                }

                return beta;
            }
            alpha = score;
            // best_action = move_list.moves[move_num].action;
        }
    }

    //if there were no captures return alpha/evaluation
    return alpha;
}

pub fn tree_search(board: &mut BoardStructure, info: &mut SearchInfo, book: &PolyglotBook, config: &ConfigInfo) {
    
    let mut best_action: usize = 0;

    let mut best_score: i32;
    let mut pv_moves: usize;

    clear_info(board, info);

    // check for book move
    if config.use_opening_book {
        best_action = get_book_move(board, book);
    }

    // if no book move was found apply standard iterative deepening
    if best_action == 0 {
        //iterative deepening
        for current_depth in 1..(info.depth_lim + 1) {

            //apply alpha beta search
            best_score = alpha_beta(board, info, -INF, INF, current_depth, config);

            //stop if out of time
            if info.stopped {
                break;
            }

            get_pv_line(board, current_depth);
            best_action = board.pv_line[0];

            // print info (for UCI)
            print!("info score cp {} depth {} nodes {} time {} ",
                best_score, current_depth, info.nodes, get_time() - info.start_time);

            pv_moves = get_pv_line(board, current_depth);

            print!("pv");
            for pv_num in 0..current_depth {
                print!(" ");
                vis_mv(board.pv_line[pv_num]);
            }
            print!("\n");
            if config.fhfquot {
                print!("Ordering: {}\n", info.fail_high_fst/info.fail_high);
            }
        }
    }

    //for UCI
    print!("bestmove ");
    vis_mv(best_action);
    print!("\n");

}





// fn parallel_alpha_beta(board: &mut BoardStructure, info: &mut SearchInfo, depth: usize) -> i32 {
    
//     // get pseudo legal moves
//     let mut move_list: MoveListStructure = Default::default();
//     pseudo_legal_move_gen(board, &mut move_list);

//     // build vector from copies of search info
//     let mut tuple_vec: Vec<(usize, &mut BoardStructure, &mut SearchInfo, i32, i32, usize, bool)> = Vec::with_capacity(move_list.length);
//     for move_num in 0..move_list.length {
//         tuple_vec.push(
//             (move_list.moves[move_num].action, board, info, -INF, INF, depth, true)
//         );
//     }

//     // get iterator from vector
//     let mut tuple_iter = tuple_vec.par_iter();


//     // apply alpha-beta to each tuple in iterator
//     tuple_iter.map(move |tup| {
//         let (action, mut board, mut info, mut alpha, beta, depth, boolean) = *tup;
//         let mut score: i32;
//         let mut best_action: usize;

//         //make move and check for validity
//         if make_move(action, board) {

//             score = - alpha_beta(board, info, -beta, -alpha, depth - 1, true);

//             //unmake previously made move
//             unmake_move(board);

//             //stop if we are out of time
//             if info.stopped {
//                 return 0;
//             }

//             if score > alpha {
//                 //alpha cutoff
//                 if score >= beta{
//                     //beta cutoff

//                     //killer heuristic
//                     if move_list.moves[move_num].action & CAP_FLAG == 0 {
//                         board.search_killers[board.ply][1] = board.search_killers[board.ply][0];
//                         board.search_killers[board.ply][0] = move_list.moves[move_num].action;
//                     }

//                     return beta;
//                 }
                
//                 alpha = score;
//                 best_action = action;

//                  //search history heuristic
//                 if move_list.moves[move_num].action & CAP_FLAG == 0 {
//                     board.search_history[to_sq(best_action)][board.board[from_sq(best_action)] as usize] += depth;
//                 }
//             }
//         }
//         //check for check- and stalemate
//         if legal_count == 0 {
//             if king_attacked {
//                 //checkmate
//                 return -MATE + (board.ply as i32);
//             } else {
//                 //stalemate
//                 return 0;
//             }
//         }

//         if alpha != old_alpha {
//             store_pv_move(board, best_action);
//         }

//         return alpha;
//     });


//     tuple_iter.max_by(||)

//     return 0;
   
// }