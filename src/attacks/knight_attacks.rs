use {once_cell::sync::Lazy, strum::IntoEnumIterator};

use crate::{
    attacks::AttackArray,
    bitboard::{Bitboard, NOT_AB_FILES, NOT_A_FILE, NOT_HG_FILES, NOT_H_FILE},
    square::BoardSquare,
};

/// [knight's square] -> squares it attacks
pub static KNIGHT_ATTACKS: Lazy<AttackArray> = Lazy::new(|| {
    let mut attacks = AttackArray::default();

    for square in BoardSquare::iter() {
        attacks[square] = mask_attacks(square);
    }

    attacks
});

fn mask_attacks(square: BoardSquare) -> Bitboard {
    let mut attacks = Bitboard(0);

    let knight = Bitboard::from(square);

    *attacks |= (*knight >> 6) & *NOT_AB_FILES;
    *attacks |= (*knight >> 10) & *NOT_HG_FILES;
    *attacks |= (*knight >> 15) & *NOT_A_FILE;
    *attacks |= (*knight >> 17) & *NOT_H_FILE;

    *attacks |= (*knight << 6) & *NOT_HG_FILES;
    *attacks |= (*knight << 10) & *NOT_AB_FILES;
    *attacks |= (*knight << 15) & *NOT_H_FILE;
    *attacks |= (*knight << 17) & *NOT_A_FILE;

    attacks
}
