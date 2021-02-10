/////////////////////////////////////////////////////////////////////

//internal imports:
use crate::statics::*;

//external imports:

/////////////////////////////////////////////////////////////////////

pub fn lsh(bb: u64, sh: usize) -> u64 {
    //shift a bitboard; when shift exceeds 63 bits return 0
    if sh < 64 {
        bb << sh
    } else {
        0u64
    }
}

pub fn rsh(bb: u64, sh: usize) -> u64 {
    //shift a bitboard; when shift exceeds 63 bits return 0
    if sh < 64 {
        bb >> sh
    } else {
        0u64
    }
}

pub fn set_bit(bb: &mut u64, square_index: usize) {
    //set a bit at the given square of the bitboard
    *bb |= SET_MASK[square_index];
}

pub fn clear_bit(bb: &mut u64, square_index: usize) {
    //clear a bit at the given square of the bitboard
    *bb &= CLEAR_MASK[square_index];
}


