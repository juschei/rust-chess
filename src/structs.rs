/////////////////////////////////////////////////////////////////////

//internal modules:
use crate::statics::*;

//external modules:
use std::{fs::File, io::Read};

use std::io::{self, BufRead};

/////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Copy, Clone, FromPrimitive)]
pub enum Pieces {
    Empty, //for squares without pieces
    Wp,Wn,Wb,Wr,Wq,Wk,
    Bp,Bn,Bb,Br,Bq,Bk,
    OffLimits //for squares outside of the 8x8 board
}


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Files {
    Fa, Fb, Fc, F, Fe, Ff, Fg, Fh,
    NoF //to be safe
}


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Ranks {
    R1, R2, R3, R4, R5, R6, R7, R8,
    NoR //to be safe
}


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Players {
    White,
    Black,
    Both  //representing both colors at once
}


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Squares {
    NoSq,
    A1 = 21, B1, C1, D1, E1, F1, G1, H1,
    A2 = 31, B2, C2, D2, E2, F2, G2, H2,
    A3 = 41, B3, C3, D3, E3, F3, G3, H3,
    A4 = 51, B4, C4, D4, E4, F4, G4, H4,
    A5 = 61, B5, C5, D5, E5, F5, G5, H5,
    A6 = 71, B6, C6, D6, E6, F6, G6, H6,
    A7 = 81, B7, C7, D7, E7, F7, G7, H7,
    A8 = 91, B8, C8, D8, E8, F8, G8, H8
}


//all castles still available: 1111
//only white king can castle: 1000
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CastlingRights {
    WkR = 8, WqR = 4, BkR = 2, BqR = 1
}

//keeps track of information neccessary to undo move
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Undo {
    pub action:         usize,
    pub castle_perm:    u8,
    pub en_pas:         usize,
    pub fifty_cnt:      usize,
    pub pos_key:        u64,
}


#[derive(PartialEq, Eq, Clone)]
pub struct BoardStructure {
    pub board:          [Pieces; 120],          //large board of instances of Pieces

    pub pawns:          [u64; 3],               //three bitboards describing pawns of each color

    pub piece_list:     [[usize;10];13],        //for each figure a list of squares

    pub king_sqs:       [usize; 2],             //squares of the kings

    pub side:           Players,                //current side to play
    pub castle_perm:    u8,                     //castling rights
    pub en_pas:         usize,                  //en passent square (as type usize)
    pub fifty_cnt:      usize,                  //keep track of 50 move rule

    pub ply_depth:      usize,                  //ply counter of game
    pub ply:            usize,                  //ply counter in current search

    pub pos_key:        u64,                    //key of position (useful for 3x rep)
 
    pub num_pieces:     [usize; 13],            //number of each piece type on board

    pub big_pieces:     [u8; 2],                //number of non pawn pieces on board
    pub min_pieces:     [u8; 2],                //number of minor pieces on board
    pub maj_pieces:     [u8; 2],                //number of major pieces on board

    pub mat_strength:   [usize; 2],             //material of each player

    //move ordering heuristics:
    // pub pv_table:       PVTable,                //keep track of principal variation
    pub search_history: [[usize; 13]; 120],     //keep track of already searched nodes
    pub search_killers: [[usize; 2]; 64],       //keep track of recent cut-offs

    pub pv_line:        [usize; 64],            //store the prinicpal variation line found in search

    pub history:        [Undo; 1024],           //history of relevant data to undo moves

    //transposition table
    pub trans_table:    TransTable              //keep track of already searched positions
}

//implment initalization of empty chessboard
impl Default for BoardStructure {
    fn default() -> Self {
        BoardStructure {
            board:          [Pieces::Empty; 120],
            pawns:          [0u64; 3],
            piece_list:     [[Squares::NoSq as usize; 10]; 13],
            king_sqs:       [Squares::NoSq as usize; 2],
            side:           Players::Both,
            castle_perm:    0u8,
            en_pas:         0usize,
            fifty_cnt:      0usize,
            ply_depth:      0usize,
            ply:            0usize,

            pos_key:        0u64,

            num_pieces:     [0usize; 13],
            big_pieces:     [0u8; 2],
            min_pieces:     [0u8; 2],
            maj_pieces:     [0u8; 2],

            mat_strength:   [0usize; 2],

            // pv_table:       Default::default(),
            search_history: [[0usize; 13]; 120],
            search_killers: [[0usize; 2]; 64],

            pv_line:        [0usize; 64],

            history:        [Default::default(); 1024],

            trans_table:    Default::default(),

        }
    }
}



// represent whole move as one number:
// | cast | ps | ep | prm | capt | to      | from   |
// |0     |0---|0---|00 00|00 00-|00 0000 0|000 0000|
//25 bit move:
    //-> 7 bits: from                   and_mask: 0x7F
    //-> 7 bits: to                     and_mask: 0x7F << 7
    //-> 4 bits: captured piece         and_mask: 0xF  << 14
    //-> 4 bits: promotion to           and_mask: 0xF  << 18
    //-> 1 bit: en passent capture?     or_mask:  0x400000
    //-> 1 bit: pawn start              or_mask:  0x800000
    //-> 1 bit: castling?               or_mask:  0x1000000
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MoveStructure {
    pub action: usize,
    pub score:  usize,
}

//List of possibles moves as MoveStructures
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct MoveListStructure {
    pub moves:  [MoveStructure; 255],     //we'll allow at most 255 moves per side per position
    pub length: usize,                   //number of moves stored in "MoveListStructure::moves"
}

//implement initalizer for MoveListStructure
impl Default for MoveListStructure {
    fn default() -> Self {
        MoveListStructure {
            moves: [Default::default(); 255],
            length: 0usize,
        }
    }
}

// //store information about principal variation moves
// #[derive(PartialEq, Eq, Copy, Clone, Default)]
// pub struct PVEntry {
//     pub pos_key:   u64,
//     pub action:  usize
// }

// //store PVEntires
// #[derive(PartialEq, Eq, Clone)]
// pub struct PVTable {
//     pub entries:          Vec<PVEntry>,
//     pub num_entries:      usize,
// }

// //implement the initalizer for the PVTable
// impl Default for PVTable {
//     fn default() -> Self {
//         let SIZE = 0x200000 / std::mem::size_of::<PVEntry>();

//         let mut table: PVTable = PVTable {
//             entries: Vec::with_capacity(SIZE),
//             num_entries: SIZE - 2,
//         };
//         for _ in 0..SIZE {
//             table.entries.push(Default::default());
//         }
//         return table;
//     }
// }

//flag type for transposition table
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TransFlag {
    None, Alpha, Beta, Exact
}

impl Default for TransFlag {
    fn default() -> Self {
        TransFlag::None
    }
}

//stores information about a position which was already searched
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TransTableEntry {
    pub pos_key:   u64,
    pub action:     usize,
    pub score:      i32,
    pub depth:      usize,
    pub flag:       TransFlag,
}

//transposition table of already searched positions
#[derive(PartialEq, Eq, Clone)]
pub struct TransTable {
    pub entries:     Vec<TransTableEntry>,
    pub num_entries: usize
}

//implement initalizer of transposition table
impl Default for TransTable {
    fn default() -> Self {
        let SIZE = unsafe {TRANS_TABLE_SIZE / std::mem::size_of::<TransTableEntry>()};

        let mut table: TransTable = TransTable {
            entries: Vec::with_capacity(SIZE),
            num_entries: SIZE - 2,
        };
        for _ in 0..SIZE {
            table.entries.push(Default::default());
        }

        return table
    }
}



//structure to ease searching
#[derive(Copy, Clone, Default)]
pub struct SearchInfo {
    pub timed:          bool,   //is the search/game timed?
    pub start_time:     u128,   //when did the search start?
    pub stop_time:      u128,   //when should the search stop?

    pub depth_lim:      usize,  //depth limit of search
    pub ply_depth:      usize,  //current depth of search

    pub moves_to_go:    usize,
    pub inf:            usize,

    pub nodes:          usize,  //total number of nodes searched

    pub quit:           bool,  
    
    pub stopped:        bool,   //indicates if search has stopped (due to time out for example)

    pub fail_high:      f32,    //number of beta cutoffs
    pub fail_high_fst:  f32,    //number of beta cutoffs at first searched move
}

// data types dictated by polyglot book format
pub struct PolyglotBookEntry {
    pub key:    u64,
    pub action: u16,
    pub weigth: u16,
    pub learn:  u32,
}

pub struct PolyglotBook {
    pub entries:     Vec<PolyglotBookEntry>,
}

impl PolyglotBook {
    pub fn from_file(path: String) -> Self {

        let mut data = File::open(path).unwrap();

        let mut buffer: Vec<u8> = Vec::new();

        match data.read_to_end(&mut buffer) {
            Ok(_) => (),
            _     => panic!(""),
        }

        // empty book
        let mut book: PolyglotBook = PolyglotBook {
            entries: Vec::new(),
        };

        // placeholders for attributes of book entry
        let mut key: u64;
        let mut action: u16;
        let mut weigth: u16;
        let mut learn: u32;
        
        // loop through buffer and combine into attributes of 
        let mut ind: usize;
        for i in 0..(buffer.len()/16) {

            ind = 16 * i;

            key     = 0;
            action  = 0;
            weigth  = 0;
            learn   = 0;

            key |= (buffer[ind + 0] as u64) << 56;
            key |= (buffer[ind + 1] as u64) << 48;
            key |= (buffer[ind + 2] as u64) << 40;
            key |= (buffer[ind + 3] as u64) << 32;
            key |= (buffer[ind + 4] as u64) << 24;
            key |= (buffer[ind + 5] as u64) << 16;
            key |= (buffer[ind + 6] as u64) <<  8;
            key |= (buffer[ind + 7] as u64) <<  0;

            action |= (buffer[ind + 8 + 0] as u16) << 8;
            action |= (buffer[ind + 8 + 1] as u16) << 0;

            weigth |= (buffer[ind + 10 + 0] as u16) << 8;
            weigth |= (buffer[ind + 10 + 1] as u16) << 0;

            learn |= (buffer[ind + 12 + 0] as u32) << 24;
            learn |= (buffer[ind + 12 + 1] as u32) << 16;
            learn |= (buffer[ind + 12 + 2] as u32) <<  8;
            learn |= (buffer[ind + 12 + 3] as u32) <<  0;

            book.entries.push(
                PolyglotBookEntry {
                    key:    key,
                    action: action,
                    weigth: weigth,
                    learn:  learn,
                }
            );
        }

        return book;
    }
}

pub struct ConfigInfo {
    pub use_opening_book:       bool,
    pub opening_book_path:      String,

    pub trans_table_size:       usize,

    pub use_endgame_database:   bool,
    pub endgame_database_path:  String,
    
    pub fifty_null:             bool,
    pub threefold_null:         bool,
    pub fhfquot:                bool,
}

impl ConfigInfo {
    pub fn from_config() -> Self {

        let mut info = ConfigInfo {
            use_opening_book:       true,
            opening_book_path:      "".to_string(),
            
            trans_table_size:       0,

            use_endgame_database:   false,
            endgame_database_path:  "".to_string(),

            fifty_null:             false,
            threefold_null:         false,
            fhfquot:                false,

        };

        let file = File::open("config.txt").unwrap();
        let lines = io::BufReader::new(file).lines();

        let mut line_string: String;
        let mut line_len: usize;

        println!("\nSetting configurations for engine:\n");

        for line in lines {
            line_string = line.unwrap();
            line_len    = line_string.len();

            if line_len >= 18 && line_string[..18].eq("USE OPENING BOOK: ") {
                // USE OPENING BOOK?
                info.use_opening_book = line_string[18..].eq("true");
                println!("USE OPENING BOOK: {}", info.use_opening_book);

            } else if line_len >= 19 && line_string[..19].eq("OPENING BOOK PATH: ") {
                // PATH TO OPENING BOOK
                info.opening_book_path = line_string[19..].to_string();
                println!("OPENING BOOK PATH: {}", info.opening_book_path);

            } else if line_len >= 31 && line_string[..31].eq("ADD THREEFOLD REPETITION NULL: ") {
                // CHECK THREEFOLD REP?
                info.threefold_null = line_string[31..].eq("true");
                println!("ADD THREEFOLD REP NULL: {}", info.threefold_null);

            } else if line_len >= 19 && line_string[..19].eq("ADD 50 COUNT NULL: ") {
                // CHECK 50 MOVE RULE?
                info.fifty_null = line_string[19..].eq("true");
                println!("ADD 50 COUNT NULL: {}", info.fifty_null);
            } else if line_len >= 32 && line_string[..32].eq("PRINT FAIL HIGH FIRST QUOTIENT: ") {
                info.fhfquot = line_string[32..].eq("true");
                println!("PRINT FAIL HIGH FIRST QUOTIENT: {}", info.fhfquot);
            } else if line_len >= 6 && line_string[..6].eq("SIZE: ") {
                info.trans_table_size = crate::uci::parse_num(line_string[6..].chars().collect()).0.unwrap();
                unsafe {TRANS_TABLE_SIZE = info.trans_table_size;};
                println!("TRANS TABLE SIZE: {} bytes", info.trans_table_size);
            }
        }
        println!("");

        return info;
    }
}