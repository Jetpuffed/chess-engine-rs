//! <Module description goes here>

use super::Bitboard;

pub const KNIGHT_ATTACK_LUT: [Bitboard; 0] = [];
pub const BISHOP_ATTACK_LUT: [Bitboard; 0] = [];
pub const ROOK_ATTACK_LUT: [Bitboard; 0] = [];
pub const KING_ATTACK_LUT: [Bitboard; 0] = [];

pub const KNIGHT_MASK_LUT: [Bitboard; 0] = [];
pub const BISHOP_MASK_LUT: [Bitboard; 0] = [];
pub const ROOK_MASK_LUT: [Bitboard; 0] = [];
pub const KING_MASK_LUT: [Bitboard; 0] = [];

pub fn create_knight_attack_lut() { todo!() }

pub fn create_bishop_attack_lut() { todo!() }

pub fn create_rook_attack_lut() { todo!() }

pub fn create_king_attack_lut() { todo!() }

/// Generates all legal knight moves.
pub fn create_knight_mask_lut() -> [Bitboard; 64]
{
    const NOT_FILE_A: Bitboard = 0x7F7F7F7F7F7F7F7F;
    const NOT_FILE_AB: Bitboard = 0x3F3F3F3F3F3F3F3F;
    const NOT_FILE_H: Bitboard = 0xFEFEFEFEFEFEFEFE;
    const NOT_FILE_GH: Bitboard = 0xFCFCFCFCFCFCFCFC;

    let mut table: [Bitboard; 64] = [0; 64];

    for (i, square) in table.iter_mut().enumerate()
    {
        let knight = 1 << i;
        let (l1, l2) = (((knight >> 1) & NOT_FILE_A), ((knight >> 2) & NOT_FILE_AB));
        let (r1, r2) = (((knight << 1) & NOT_FILE_H), ((knight << 2) & NOT_FILE_GH));
        let (h1, h2) = ((l1 | r1), (l2 | r2));

        *square = (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8);
    }

    table
}

pub fn create_bishop_mask_lut() { todo!() }

pub fn create_rook_mask_lut() { todo!() }

pub fn create_king_mask_lut() { todo!() }
