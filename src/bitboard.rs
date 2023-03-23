use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitOrAssign, Deref, DerefMut, Shl, Shr},
};

use crate::square::BoardSquare;

#[derive(Copy, Clone, Default)]
pub struct Bitboard(pub u64);

impl Deref for Bitboard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bitboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BoardSquare> for Bitboard {
    fn from(value: BoardSquare) -> Self {
        Self(1 << value as u8)
    }
}

impl Bitboard {
    /// Returns true if the bit at the given position is set
    pub fn get_bit(&self, idx: usize) -> bool {
        self.0 & (1 << idx) > 0
    }

    /// Make the bit at `idx` 1
    pub fn set_bit(&mut self, idx: usize) {
        self.0 |= 1 << idx
    }

    /// Make the biy at `idx` 0
    pub fn pop_bit(&mut self, idx: usize) {
        self.0 &= !(1 << idx)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n      Bitboard: {}\n", self.0)?;
        for rank in 0..8 {
            for file in 0..8 {
                if file == 0 {
                    write!(f, "  {} ", 8 - rank)?;
                }

                let square = rank * 8 + file;
                write!(f, " {}", if self.get_bit(square) { 1 } else { 0 })?;
            }

            writeln!(f)?;
        }
        writeln!(f, "\n     a b c d e f g h")?;

        Ok(())
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= *rhs;
    }
}
impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(*self & *rhs)
    }
}
impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(*self | *rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0.shr(rhs))
    }
}
impl Shl<usize> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0.shl(rhs))
    }
}

/// Everything is set to 1 except for the A file
pub const NOT_A_FILE: Bitboard = Bitboard(18374403900871474942);
/// Everything is set to 1 except for the H file
pub const NOT_H_FILE: Bitboard = Bitboard(9187201950435737471);
/// Everything is set to 1 except for the H and G files
pub const NOT_HG_FILES: Bitboard = Bitboard(4557430888798830399);
/// Everything is set to 1 except for the A and B files
pub const NOT_AB_FILES: Bitboard = Bitboard(18229723555195321596);
