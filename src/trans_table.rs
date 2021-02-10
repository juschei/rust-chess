/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::structs::*;
use crate::statics::*;
use crate::legal_move_gen::*;
use crate::do_undo_moves::*;

//external modules:

/////////////////////////////////////////////////////////////////////

pub fn store_trans_move(board: &mut BoardStructure, action: usize, mut score: i32, depth: usize, flag: TransFlag) {
    let index: usize = (board.pos_key as usize) % board.trans_table.num_entries;

    //reset mate counter for later adjustment
    if score > IS_MATE {
        score += board.ply as i32;
    } else if score < - IS_MATE {
        score -= board.ply as i32;
    }

    board.trans_table.entries[index] = TransTableEntry {
        pos_key:    board.pos_key,
        action:     action,
        score:      score,
        depth:      depth,
        flag:       flag
    }
}



pub fn probe_trans_table(board: &BoardStructure, action: &mut usize, score: &mut i32, alpha: i32, beta: i32, depth: usize) -> bool {
    let index: usize = (board.pos_key as usize) % board.trans_table.num_entries;

    let entry: TransTableEntry = board.trans_table.entries[index];

    if board.trans_table.entries[index].pos_key == board.pos_key {
        *action = entry.action;

        if entry.depth >= depth {
            *score = entry.score;

            //readjust mate counter to new depth
            if *score > IS_MATE {
                *score -= board.ply as i32;
            } else if *score < - IS_MATE {
                *score += board.ply as i32
            }

            //check for cutoff flags
            match entry.flag {
                TransFlag::None  => panic!("No Flag stored!"),
                TransFlag::Alpha => {
                    if *score <= alpha {
                        *score = alpha;
                        return true;
                    }
                },
                TransFlag::Beta  => {
                    if *score >= beta {
                        *score = beta;
                        return true;
                    }
                },
                TransFlag::Exact => {
                    return true;
                },
            }
        }
    }
    return false;
}