mod bitboard;
pub mod square;

use bitboard::Bitboard;
use square::BoardSquare;

fn main() {
    let mut bb = Bitboard(4);

    bb.set_bit(BoardSquare::B4 as usize);
    bb.set_bit(BoardSquare::C3 as usize);

    println!("{bb}");
    bb.pop_bit(BoardSquare::C3 as usize);
    println!("{bb}");
    bb.pop_bit(BoardSquare::C3 as usize);
    println!("{bb}");
}
