mod attacks;
mod bitboard;
pub mod square;

use strum::IntoEnumIterator;

use crate::{attacks::KNIGHT_ATTACKS, bitboard::Bitboard, square::BoardSquare};

fn main() {
    for square in BoardSquare::iter() {
        let attacks = KNIGHT_ATTACKS[square];
        println!("{}", Bitboard(*attacks | *Bitboard::from(square)));
    }
}
