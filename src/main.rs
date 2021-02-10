/////////////////////////////////////////////////////////////////////

//REMOVE ALLOWANCES WHEN CODE IS DONE TO GET USEFUL LINTS
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_attributes)]

/////////////////////////////////////////////////////////////////////

//powershell backtrace var:
//$Env:RUST_BACKTRACE=1

/////////////////////////////////////////////////////////////////////

//internal modules
mod bitboards;
mod visualization;
mod statics;
mod structs;
mod legal_move_gen;
mod do_undo_moves;
mod perft;
mod fen;
mod hash;
mod validation;
mod search;
mod principal_variation;
mod trans_table;
mod evaluation;
mod uci;
mod polyglot;
mod console_game;


//internal imports
use crate::uci::*;
use crate::console_game::*;

//external imports
extern crate lazy_static;
extern crate rand;
extern crate num;
extern crate unicode_segmentation;

#[macro_use]
extern crate num_derive;

/////////////////////////////////////////////////////////////////////

/*

TODO:
    threefold repetition
    50 moves
    
*/

/////////////////////////////////////////////////////////////////////

fn main() {
    let _fen_1: String = String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    let _fen_2: String = String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2");
    let _fen_3: String = String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
    let _fen_4: String = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let _wpawn_test_fen: String = String::from("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
    let _bpawn_test_fen: String = String::from("rnbqkbnr/p1p1p3/3p3p/1p1p4/2P1Pp2/8/PP1P1PpP/RNBQKB1R b KQkq e3 0 1");
    let _kingknight_fen: String = String::from("5k2/1n6/4n3/6N1/8/4N4/8/5K2 b - - 0 1"); 
    let _bishop_fen:String = String::from("6k1/1b6/4n3/8/1n4B1/1B3N2/1N6/2b3K1 b - - 0 1");
    let _rook_fen: String = String::from("6k1/8/5r2/8/1nR5/5N2/8/6K1 w - - 0 1");
    let _queen_fen:String = String::from("6k1/8/4nq2/8/1nQ5/5N2/1N6/6K1 w - - 0 1");
    let _castling_fen: String = String::from("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1");
    let _castling_2_fen: String = String::from("3rk2r/8/8/8/8/8/6p1/R3K2R w KQk - 0 1");
    let _castling_3_fen: String = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let _mate_in_three: String = String::from("2rr3k/pp3pp1/1nnqbN1p/3pN3/2pP4/2P3Q1/PPB4P/R4RK1 w - -");
    let _wac1: String = String::from("r1b1k2r/ppppnppp/2n2q2/2b5/3NP3/2P1B3/PP3PPP/RN1QKB1R w KQkq - 0 1");


    // Eden(START_FEN.to_string(), 7, "src/performance.bin".to_string());

    uci_loop();

}





