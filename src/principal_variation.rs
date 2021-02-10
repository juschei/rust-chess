/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::structs::*;
use crate::legal_move_gen::*;
use crate::do_undo_moves::*;

//external modules:

/////////////////////////////////////////////////////////////////////

// pub fn store_pv_move(board: &mut BoardStructure, action: usize) {
//     let index: usize = (board.pos_key as usize) % board.pv_table.num_entries;

//     board.pv_table.entries[index].action = action;
//     board.pv_table.entries[index].pos_key = board.pos_key;
// }

pub fn probe_table(board: &BoardStructure) -> usize {
    let index: usize = (board.pos_key as usize) % board.trans_table.num_entries;

    if board.trans_table.entries[index].pos_key == board.pos_key {
        return board.trans_table.entries[index].action;
    } else {
        return 0usize;
    }
}

pub fn move_exists(action: usize, board: &mut BoardStructure) -> bool {

    let mut move_list: MoveListStructure = Default::default();
    pseudo_legal_move_gen(board, &mut move_list);

    for move_num in 0..move_list.length {
        if !make_move(move_list.moves[move_num].action, board) {
            continue;
        }
        unmake_move(board);
        if move_list.moves[move_num].action == action {
            return true;
        }
    }
    return false;
}

pub fn get_pv_line(board: &mut BoardStructure, depth: usize) -> usize {

    let mut action: usize = probe_table(board);
    let mut count: usize = 0;

    while action != 0 && count < depth {
        if move_exists(action, board) {
            make_move(action, board);
            board.pv_line[count] = action;
            count += 1; 
        } else {
            break;
        }
        action = probe_table(board);
    }

    while board.ply > 0 {
        unmake_move(board);
    }

    return count;

}