mod attacks;
mod bitboard;
pub mod square;

use {
    attacks::PAWN_ATTACKS,
    square::{BoardSquare, Color},
};

fn main() {
    let attacks = PAWN_ATTACKS[Color::White][BoardSquare::E8];
    println!("{attacks}");
}
