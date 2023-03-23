use {once_cell::sync::Lazy, strum::IntoEnumIterator};

use crate::{
    attacks::ColorAttackArray,
    bitboard::{Bitboard, NOT_A_FILE, NOT_H_FILE},
    square::{BoardSquare, Color},
};

/// [color][pawn square] -> squares it attacks
pub static PAWN_ATTACKS: Lazy<ColorAttackArray> = Lazy::new(|| {
    let mut pawn_attacks = ColorAttackArray::default();

    for square in BoardSquare::iter() {
        pawn_attacks[Color::White][square] = mask_attacks(square, Color::White);
        pawn_attacks[Color::Black][square] = mask_attacks(square, Color::Black);
    }

    pawn_attacks
});

fn mask_attacks(pawn_square: BoardSquare, to_play: Color) -> Bitboard {
    // Result attacks bb
    let mut attacks = Bitboard(0);

    // Piece bb
    let bb = Bitboard::from(pawn_square);

    match to_play {
        Color::White => {
            // If the attack to the right falls on the A file
            // it means that we are on the H file and it wrapped,
            // so don't count it
            let right_attack = *bb >> 7;
            *attacks |= right_attack & *NOT_A_FILE;

            // Same as above but for the left
            let left_attack = *bb >> 9;
            *attacks |= left_attack & *NOT_H_FILE;
        }
        Color::Black => {
            let right_attack = *bb << 7;
            *attacks |= right_attack & *NOT_H_FILE;

            let left_attack = *bb << 9;
            *attacks |= left_attack & *NOT_A_FILE;
        }
    }

    attacks
}
