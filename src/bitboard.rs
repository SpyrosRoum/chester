use std::fmt::Display;

use crate::square::BoardSquare;

pub struct Bitboard(pub u64);

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
