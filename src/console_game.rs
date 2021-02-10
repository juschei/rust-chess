/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::structs::*;
use crate::visualization::*;
use crate::fen::*;
use crate::do_undo_moves::*;
use crate::search::*;
use crate::perft::*;
use crate::uci::*;

//external modules:
use std::io;

/////////////////////////////////////////////////////////////////////

pub fn Eden(fen: String, depth: usize) {
    
    let mut board: BoardStructure = Default::default();
    let mut info: SearchInfo = Default::default();

    let config: &ConfigInfo = &ConfigInfo::from_config();

    let book: PolyglotBook = PolyglotBook::from_file(config.opening_book_path.clone());

    //set search depth for Eden and general search
    info.depth_lim = depth;

    build_fen(fen, &mut board);

    let start_eden: bool = ask_game_mode();

    if start_eden {

        let mut edens_action: usize;

        //enter game loop
        loop {
            vis_board(&board);
            if board.side == Players::White {
                ask_and_make_user_action(&mut board, &mut info, &book, config);
            } else {
                println!("Now Eden:");
                tree_search(&mut board, &mut info, &book, config);
                edens_action = board.pv_line[0];
                make_move(edens_action, &mut board);
            }
        }
    } else {
        loop {
            vis_board(&board);
            ask_and_make_user_action(&mut board, &mut info, &book, config);
        }
    }
}


pub fn ask_game_mode() -> bool {

    let mut input: String = String::new();

    println!("Would you like to play against Eden? (Yes/No)");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.contains("Yes") {
                return true;
            } else if input.contains("No") {
                return false;
            } else {
                println!("Please try again!");
                return ask_game_mode();
            }
        },
        _     => {
            println!("Something went wrong, plese restart the program!");
            std::process::exit(0);
        },
    }
}

pub fn ask_and_make_user_action(board: &mut BoardStructure, info: &mut SearchInfo, book: &PolyglotBook, config: &ConfigInfo) {

    //loop until valid input is given
    loop {
        let mut input = String::new();

        println!("Please enter a move: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.contains("takeback") {
                    unmake_move(board);
                    break;
                } else if input.contains("2xtakeback") {
                    unmake_move(board);
                    unmake_move(board);
                    break;
                } else if input.contains("perft") {
                    medium_perft(board, info.depth_lim);
                } else if input.contains("search") {
                    info.start_time = get_time();
                    info.stop_time = get_time() + 10;
                    tree_search(board, info, &book, config);
                } else if input.contains("quit") {
                    std::process::exit(0);
                } else {
                    let action: usize = parse_move(input, board);

                    if action == 0 {
                        println!("Oops, please try again!");
                        continue;
                    } else {
                        make_move(action, board);
                        break;
                    }
                }
            },
            _     => println!("Input was not understood"),
        }
    }
}