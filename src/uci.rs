/////////////////////////////////////////////////////////////////////

//internal imports
use crate::structs::*;
use crate::statics::*;
use crate::fen::*;
use crate::do_undo_moves::*;
use crate::visualization::*;
use crate::legal_move_gen::*;
use crate::search::*;
use crate::polyglot::*;

//external imports

/////////////////////////////////////////////////////////////////////

pub fn parse_move(action: String, board: &BoardStructure) -> usize {
    //take string of move in long algebraic notation and return corresponding move

    let mut actioncopy: String = action.chars().rev().collect::<String>();

    let file_from: usize = actioncopy.pop().unwrap() as usize - 'a' as usize;
    let rank_from: usize = actioncopy.pop().unwrap() as usize - '1' as usize;
    let file_to: usize = actioncopy.pop().unwrap() as usize - 'a' as usize;
    let rank_to: usize = actioncopy.pop().unwrap() as usize - '1' as usize;

    let from: usize = (21 + file_from) + (rank_from * 10);
    let to: usize = (21 + file_to) + (rank_to * 10);

    let mut movelist: MoveListStructure = Default::default();

    pseudo_legal_move_gen(board, &mut movelist);


    let mut last: char = 't';
    match actioncopy.pop() {
        Some(chr) => last = chr,
        _         => (),
    }

    for move_num in 0..movelist.length {
        let mv = movelist.moves[move_num].action;
        if from_sq(mv) == from && to_sq(mv) == to {
            let promoted: usize = promoted(mv);
            if promoted != Pieces::Empty as usize {
                if IS_ROOK_QUEEN_MASK[promoted] && !IS_BISHOP_QUEEN_MASK[promoted] && last == 'r' {
                    return mv;
                } else if !IS_ROOK_QUEEN_MASK[promoted] && IS_BISHOP_QUEEN_MASK[promoted] && last == 'b' {
                    return mv;
                } else if IS_ROOK_QUEEN_MASK[promoted] && IS_BISHOP_QUEEN_MASK[promoted] && last == 'q' {
                    return mv;
                } else if IS_KNIGHT_MASK[promoted] && last == 'n' {
                    return mv;
                }
                continue;
            }
            return mv;
        }
    }

    return 0usize;

}

fn parse_position(input_chars: Vec<char>, board: &mut BoardStructure) {

    //start reading array at position 10
    let mut head: usize = 9;
    let mut rem_chars: Vec<char> = input_chars;
    let rem_string: String;

    //check for "startpos" order
    if rem_chars[head..(head+8)] == ['s','t','a','r','t','p','o','s'] {
        build_fen(START_FEN.to_string(), board);
        head += 9;

    } else {
        //"fen" order

        //go to fen
        head += 4;

        //get remaining chars where fen is prefix
        rem_chars = rem_chars[head..].to_vec();

        //build_fen only cares if fen to process is prefix of string
        build_fen(rem_chars.iter().cloned().collect(), board);
    }

    //check for "moves" order
    let moves_given: bool;
    //update string
    rem_string = rem_chars.iter().cloned().collect();
    match rem_string.find("moves ") {
        Some (index) => {
            //go to first move
            head = index + 6;
            //update indicator
            moves_given = true;
        },
        _ => {
            //update indicator
            moves_given = false;
        },
    }

    if moves_given {
        //parse moves that are given

        //cut char list
        rem_chars = rem_chars[head..].to_vec();

        //update head
        head = 0;

        //variable to store move
        let mut action: usize;

        while head + 4 < rem_chars.len() {
            //check for promotion
            
            if rem_chars[head+4] != ' ' && rem_chars[head+4] != '\r' && rem_chars[head+4] != '\n' {
                //promotion

                //get and make move
                action = parse_move(rem_chars[head..(head+5)].iter().cloned().collect(), board);
                make_move(action, board);
                board.ply = 0;

                //get to next move
                head += 6;

                continue;

            } else {
                //non promotion || end of "moves" order

                //get and make move
                action = parse_move(rem_chars[head..(head+4)].iter().cloned().collect(), board);
                make_move(action, board);
                board.ply = 0;

                //get to next move || walk out of bounds
                head += 5;

                continue;
            }
        }
    }

    //check for moves to make
    vis_board(board);
}

pub fn parse_num(chars: Vec<char>) -> (Option<usize>, usize) {
    //parse a vector of chars to a number and ignoring non numeral tails,
    //return the number and its length in decimal
    let mut head: usize = 0;
    let mut num: usize = 0;

    let mut digit_found: bool;

    while head < chars.len() {

        //rset indicator
        digit_found = false;

        //check containment
        for digit in ['0','1','2','3','4','5','6','7','8','9'].iter() {
            if chars[head] == *digit {
                //update mnumber
                num = 10 * num + (*digit).to_digit(10).unwrap() as usize;
                //move head
                head += 1;
                //update indicator
                digit_found = true;
                //break digit check
                break;
            }
        }

        if !digit_found {
            break;
        }
    }

    if head == 0 {
        return (None, 0usize);
    } else {
        return (Some(num), head);
    }
}

fn parse_go(input_chars: Vec<char>, info: &mut SearchInfo, board: &mut BoardStructure, book: &PolyglotBook, config: &ConfigInfo) {

    let mut num: (Option<usize>, usize);

    let mut inc: u128 = 0;
    let mut time: u128 = 0;
    let mut movetime: u128 = 0;
    let mut movestogo: usize = 30;
    let mut depth: usize = 0;

    let mut time_ind: bool = false;
    let mut movetime_ind: bool = false;
    let mut depth_ind: bool = false;

    info.timed = false;

    //start head at first order of "go"
    let mut head: usize = 3;
    let max_head: usize = input_chars.len();

    if head+5 < max_head && input_chars[head..(head+5)] == ['w','t','i','m','e'] {

        //update head
        head += 6;

        //get number
        num = parse_num(input_chars[head..].to_vec());

        if board.side == Players::White {
            //get time
            time = num.0.unwrap() as u128;

            //update indicator
            time_ind = true;
        }

        //get past number
        head += num.1 + 1;
    }

    // println!("head: {}, char: {}", head, input_chars[head]);
    // let mut print_string = &input_chars[head..(head+5)];
    // println!("string: {:?}", print_string);

    if head+5 < max_head && input_chars[head..(head+5)] == ['b','t','i','m','e'] {

        //update head
        head += 6;

        //get number
        num = parse_num(input_chars[head..].to_vec());

        if board.side == Players::Black {
            //get time
            time = num.0.unwrap() as u128;

            //update indicator
            time_ind = true;
        }

        //get past number
        head += num.1 + 1;
    }

    // println!("head: {}, char: {}", head, input_chars[head]);
    // let mut print_string = &input_chars[head..(head+4)];
    // println!("string: {:?}", print_string);

    if head+4 < max_head && input_chars[head..(head+4)] == ['w','i','n','c'] {

        //update head
        head += 5;

        //get number
        num = parse_num(input_chars[head..].to_vec());

        if board.side == Players::White {
            //get increment
            inc = num.0.unwrap() as u128;
        }

        //get past number
        head += num.1 + 1;
    }

    // println!("head: {}, char: {}", head, input_chars[head]);
    // let mut print_string = &input_chars[head..(head+4)];
    // println!("string: {:?}", print_string);

    if head+4 < max_head && input_chars[head..(head+4)] == ['b','i','n','c'] {

        //update head
        head += 5;

        //get number
        num = parse_num(input_chars[head..].to_vec());

        if board.side == Players::Black {
            //get increment
            inc = num.0.unwrap() as u128;
        }

        //get past number
        head += num.1 + 1;
    }

    // println!("head: {}, char: {}", head, input_chars[head]);
    // let mut print_string = &input_chars[head..(head+9)];
    // println!("string: {:?}", print_string);

    if head+9 < max_head && input_chars[head..(head+9)] == ['m','o','v','e','s','t','o','g','o'] {

        //update head
        head += 10;

        //get number
        num = parse_num(input_chars[head..].to_vec());

        //get moves to go
        movestogo = num.0.unwrap();

        //get past number
        head += num.1 + 1;
    }

    // println!("head: {}, char: {}", head, input_chars[head]);
    // let mut print_string = &input_chars[head..(head+5)];
    // println!("string: {:?}", print_string);

    if head+5 < max_head && input_chars[head..(head+5)] == ['d','e','p','t','h'] {

        //update head
        head += 6;

        //get number
        num = parse_num(input_chars[head..].to_vec());

        //get depth
        depth = num.0.unwrap();

        //get past number
        head += num.1 + 1;

        //update indicator
        depth_ind = true;
    }

    // println!("head: {}, char: {}", head, input_chars[head]);
    // let mut print_string = &input_chars[head..(head+8)];
    // println!("string: {:?}", print_string);

    if head+8 < max_head && input_chars[head..(head+8)] == ['m','o','v','e','t','i','m','e'] {

        //update head
        head += 9;

        //get number
        num = parse_num(input_chars[head..].to_vec());

        //get movetime
        movetime = num.0.unwrap() as u128;

        //get past number
        head += num.1 + 1;

        //update indicator
        movetime_ind = true;
    }

    // println!("head: {}, char: {}", head, input_chars[head]);
    // let mut print_string = &input_chars[head..(head+8)];
    // println!("string: {:?}", print_string);

    if head+8 < max_head && input_chars[head..(head+8)] == ['i','n','f','i','n','i','t','e'] {
        ();
    }

    if movetime_ind {
        time = movetime;
        movestogo = 1;

        //update indicator
        time_ind = true;
    }

    if depth_ind {
        info.depth_lim = depth;
    } else {
        info.depth_lim = usize::MAX - 1;
    }

    info.start_time = get_time();

    if time_ind {
        info.timed = true;
        time /= movestogo as u128;
        time -= 50;
        // info.stop_time = info.start_time + time + inc; //rem(n)=(inc+a(n-1))*(1-1/movestogo)-inc - latency, a(0)=clock_time - inc
        info.stop_time = info.start_time + time;          //rem(n)=(inc+a(n-1))*(1-1/movestogo))    - latency, a(0)=clock_time - inc
    }

    print!("time:{} start:{} stop:{} depth:{} timeset:{}\n", 
            time,
            info.start_time,
            info.stop_time,
            info.depth_lim,
            info.timed);

    tree_search(board, info, book, config);
}

pub fn uci_loop() {
    //create information stream using UCI protocol

    let config: ConfigInfo = ConfigInfo::from_config();

    let mut board: BoardStructure = Default::default();
    let mut info: SearchInfo = Default::default();

    let mut input_string: String;
    let mut input_chars: Vec<char>;

    let book: PolyglotBook = PolyglotBook::from_file(config.opening_book_path.clone());

    print!("id name {}\n", NAME.to_string());
    print!("id author {}\n", AUTHOR.to_string());
    print!("uciok\n");

    loop {

        input_string = String::new();

        match std::io::stdin().read_line(&mut input_string) {
            Ok(_) => (),
            _     => continue,
        }

        input_chars = input_string.chars().collect();

        if input_chars[0] == '\n' {
            continue;
        }


        if input_chars.len() >= 2 && input_chars[0..2] == ['g','o'] {
            parse_go(input_chars, &mut info, &mut board, &book, &config);
        } else if input_chars.len() >= 3 && input_chars[0..3] == ['u','c','i'] && input_chars[3] != 'n' {
            print!("id name {}\n", NAME.to_string());
            print!("id author {}\n", AUTHOR.to_string());
            print!("uciok\n");
        } else if input_chars.len() >= 4 && input_chars[0..4] == ['q','u','i','t'] {
            info.quit = true;
            break;
        } else if input_chars.len() >= 7 && input_chars[0..7] == ['i','s','r','e','a','d','y'] {
            print!("readyok\n");
            continue;
        } else if input_chars.len() >= 8 && input_chars[0..8] == ['p','o','l','y','m','o','v','e'] {
            println!("Choosing best book move:");
            vis_mv(get_book_move(&board, &book));
            println!("");
        } else if input_chars.len() >= 8 && input_chars[0..8] == ['p','o','s','i','t','i','o','n'] {
            parse_position(input_chars, &mut board);
        } else if input_chars.len() >= 10 && input_chars[0..10] == ['u','c','i','n','e','w','g','a','m','e'] {
            parse_position(String::from("position startpos\n").chars().collect(), &mut board);
        }
        
        if info.quit {
            break;
        }
    }
}

// pub fn threaded_uci_loop() {

//     let NAME: String = String::from("Eden");

//     print!("id name {}\n", NAME);
//     print!("id author Adam\n");
//     print!("uciok\n");

//     let mut board: BoardStructure = Default::default();
//     let mut info: SearchInfo = Default::default();
//     let mut input_string: String;
//     let mut input_chars: Vec<char>;

//     // information channel
//     let (sub_in, main_out): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

//     let input_thread = thread::spawn(move || {

//         let mut input_string = String::new();

//         match std::io::stdin().read_line(&mut input_string) {
//             Ok(_) => (),
//             _     => panic!(""),
//         }   
//     });

//     // // thread for GUI input
//     // let input_thread = thread::spawn(|| {
//     //     let mut input: String = String::new();

//     //     match std::io::stdin().read_line(&mut input) {
//     //         Ok(_) => (),
//     //         _     => panic!("");
//     //     }
//     // });
    
//     thread::sleep(Duration::from_millis(1));
//     println!("{}", NAME);

//     // thread::spawn(move || {
//     //     for _ in 0..10 {
//     //         v += 1;
//     //     }
//     //     sub_in.send(v).unwrap();

//     // });

//     // v = main_out.recv().unwrap();
//     // println!("main: v={}", v);
// }