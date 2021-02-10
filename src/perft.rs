/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::statics::*;
use crate::structs::*;
use crate::legal_move_gen::*;
use crate::visualization::*;
use crate::do_undo_moves::*;
use crate::fen::*;

//external modules:
use std::io::stdin;

/////////////////////////////////////////////////////////////////////

pub fn sub() {
    
    let mut perft_board: BoardStructure = Default::default();

    let mut perft_movelist: MoveListStructure = Default::default();
    
    build_fen(START_FEN.to_string(), &mut perft_board);

    pseudo_legal_move_gen(&perft_board, &mut perft_movelist);

    vis_board(&perft_board);

    let mut action: usize;
    for move_num in 0..perft_movelist.length {
        action = perft_movelist.moves[move_num].action;

        if !make_move(action, &mut perft_board) {
            print!("\nInvalid: ");
            vis_mv(action);
            continue;
        } 

        print!("\nPushed: ");
        vis_mv(action);
        vis_board(&perft_board);

        stdin();

        unmake_move(&mut perft_board);
        print!("\nPopped: ");
        vis_mv(action);
        vis_board(&perft_board);

        stdin();
    }
}





pub fn small_perft(perft_board: &mut BoardStructure, depth: usize) -> usize {
    //does a perfomance test on the given board up to the depth given (inclusive) 

    let none_move: MoveStructure = MoveStructure {
        action: 0usize,
        score:  0usize,
    };      
    
    let mut perft_movelist: MoveListStructure = MoveListStructure {
        moves:  [none_move; 255],
        length: 0usize,
    };

    if depth == 0 {
        return 1;
    }

    pseudo_legal_move_gen(perft_board, &mut perft_movelist);

    let mut leaf_nodes: usize = 0;

    for move_num in 0..perft_movelist.length {

        if !make_move(perft_movelist.moves[move_num].action, perft_board) {
            continue
        }

        leaf_nodes += small_perft(perft_board, depth - 1);
        unmake_move(perft_board);
    }

    return leaf_nodes;
}

pub fn medium_perft(perft_board: &mut BoardStructure, depth: usize) {
    //does a perfomance test on the given board up to the depth given (inclusive)
    //print for each depth the number of cumulative nodes 

    let start = std::time::SystemTime::now();
    let since_epoch = start.duration_since(std::time::UNIX_EPOCH).expect("");
    let start_time = since_epoch.as_millis();

    let mut leaf_nodes: usize = 0;

    let mut perft_movelist: MoveListStructure = Default::default();
    vis_board(perft_board);
    print!("\nStarting test to depth: {}\n", depth);

    if depth == 0 {
        print!("Depth 0?");
    }

    pseudo_legal_move_gen(&perft_board, &mut perft_movelist);

    for move_num in 0..perft_movelist.length {
        if !make_move(perft_movelist.moves[move_num].action, perft_board) {
            continue
        }

        let cum_nodes: usize = leaf_nodes;

        let sub_leaf_nodes = small_perft(perft_board, depth - 1);
        unmake_move(perft_board);

        leaf_nodes += sub_leaf_nodes;

        let old_nodes: usize = leaf_nodes - cum_nodes;

        print!("move {}: ", move_num + 1);
        vis_mv(perft_movelist.moves[move_num].action);
        print!(" : {} leafnodes.\n", old_nodes); 
    }

    let end = std::time::SystemTime::now();
    let since_epoch = end.duration_since(std::time::UNIX_EPOCH).expect("");
    let end_time = since_epoch.as_millis();

    print!("\nTest complete: {} nodes visited in {} milliseconds:\n", leaf_nodes, end_time - start_time);
}