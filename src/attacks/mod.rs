mod knight_attacks;
mod pawn_attacks;

use std::ops::{Index, IndexMut};

use crate::{
    bitboard::Bitboard,
    square::{BoardSquare, Color},
};
pub use knight_attacks::KNIGHT_ATTACKS;
pub use pawn_attacks::PAWN_ATTACKS;

/// [square] -> squares it attacks
pub struct AttackArray([Bitboard; 64]);

/// [color][square] -> squares it attacks
#[derive(Default)]
pub struct ColorAttackArray([AttackArray; 2]);

impl Index<Color> for ColorAttackArray {
    type Output = AttackArray;

    fn index(&self, index: Color) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Color> for ColorAttackArray {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Default for AttackArray {
    fn default() -> Self {
        Self([Bitboard(0); 64])
    }
}

impl Index<BoardSquare> for AttackArray {
    type Output = Bitboard;

    fn index(&self, index: BoardSquare) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<BoardSquare> for AttackArray {
    fn index_mut(&mut self, index: BoardSquare) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
