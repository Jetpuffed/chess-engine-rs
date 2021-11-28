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

/// Generates all legal bishop moves.
pub fn create_bishop_mask_lut() -> [Bitboard; 64]
{
    const DIAGONAL: Bitboard = 0x8040201008040201;
    const ANTIDIAG: Bitboard = 0x0102040810204080;

    let mut table: [Bitboard; 64] = [0; 64];

    for (i, square) in table.iter_mut().enumerate()
    {
        let bishop = 1 << i;
        let (d1, d2) = ((8 * (i & 7) - (i & 56)), (56 - (8 * (i & 7) - (i & 56))));
        let (n1, n2) = (!(d1 - 1) & (d1 >> 31), !(d2 - 1) & (d2 >> 31));
        let (s1, s2) = (d1 & (!(d1 - 1) >> 31), d2 & (!(d2 - 1) >> 31));

        *square = bishop ^ (((DIAGONAL >> s1) << n1) | ((ANTIDIAG >> s2) << n2));
    }

    table
}

/// Generates all legal rook moves.
pub fn create_rook_mask_lut() -> [Bitboard; 64]
{
    const RANK: Bitboard = 0x00000000000000FF;
    const FILE: Bitboard = 0x0101010101010101;

    let mut table: [Bitboard; 64] = [0; 64];

    for (i, square) in table.iter_mut().enumerate()
    {
        let rook = 1 << i;
        let (r, f) = (RANK << (i & 56), FILE << (i & 7));

        *square = rook ^ (r | f);
    }

    table
}

/// Generates all legal king moves.
pub fn create_king_mask_lut() -> [Bitboard; 64]
{
    const NOT_FILE_A: Bitboard = 0x7F7F7F7F7F7F7F7F;
    const NOT_FILE_H: Bitboard = 0xFEFEFEFEFEFEFEFE;

    let mut table: [Bitboard; 64] = [0; 64];

    for (i, square) in table.iter_mut().enumerate()
    {
        let king = 1 << i;
        let (n, s) = (king << 8, king >> 8);
        let (e, w) = ((king << 1) & NOT_FILE_A, (king >> 1) & NOT_FILE_H);
        let (ne, sw) = ((king << 9) & NOT_FILE_A, (king >> 9) & NOT_FILE_H);
        let (nw, se) = ((king << 7) & NOT_FILE_A, (king >> 7) & NOT_FILE_H);

        *square = king ^ (n | s | e | w | ne | sw | nw | se);
    }

    table
}
