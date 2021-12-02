//! <Module description goes here>

use super::Bitboard;

pub const KNIGHT_MASK_LUT: [Bitboard; 64] =
[
    0x0000000000020400, 0x0000000000050800, 0x00000000000A1100, 0x0000000000142200, 0x0000000000284400, 0x0000000000508800, 0x0000000000A01000, 0x0000000000402000,
    0x0000000002040004, 0x0000000005080008, 0x000000000A110011, 0x0000000014220022, 0x0000000028440044, 0x0000000050880088, 0x00000000A0100010, 0x0000000040200020,
    0x0000000204000402, 0x0000000508000805, 0x0000000A1100110A, 0x0000001422002214, 0x0000002844004428, 0x0000005088008850, 0x000000A0100010A0, 0x0000004020002040,
    0x0000020400040200, 0x0000050800080500, 0x00000A1100110A00, 0x0000142200221400, 0x0000284400442800, 0x0000508800885000, 0x0000A0100010A000, 0x0000402000204000,
    0x0002040004020000, 0x0005080008050000, 0x000A1100110A0000, 0x0014220022140000, 0x0028440044280000, 0x0050880088500000, 0x00A0100010A00000, 0x0040200020400000,
    0x0204000402000000, 0x0508000805000000, 0x0A1100110A000000, 0x1422002214000000, 0x2844004428000000, 0x5088008850000000, 0xA0100010A0000000, 0x4020002040000000,
    0x0400040200000000, 0x0800080500000000, 0x1100110A00000000, 0x2200221400000000, 0x4400442800000000, 0x8800885000000000, 0x100010A000000000, 0x2000204000000000,
    0x0004020000000000, 0x0008050000000000, 0x00110A0000000000, 0x0022140000000000, 0x0044280000000000, 0x0088500000000000, 0x0010A00000000000, 0x0020400000000000,
];
pub const BISHOP_MASK_LUT: [Bitboard; 64] =
[
    0x0040201008040200, 0x0000402010080400, 0x0000004020100A00, 0x0000000040221400, 0x0000000002442800, 0x0000000204085000, 0x0000020408102000, 0x0002040810204000,
    0x0020100804020000, 0x0040201008040000, 0x00004020100A0000, 0x0000004022140000, 0x0000000244280000, 0x0000020408500000, 0x0002040810200000, 0x0004081020400000,
    0x0010080402000200, 0x0020100804000400, 0x004020100A000A00, 0x0000402214001400, 0x0000024428002800, 0x0002040850005000, 0x0004081020002000, 0x0008102040004000,
    0x0008040200020400, 0x0010080400040800, 0x0020100A000A1000, 0x0040221400142200, 0x0002442800284400, 0x0004085000500800, 0x0008102000201000, 0x0010204000402000,
    0x0004020002040800, 0x0008040004081000, 0x00100A000A102000, 0x0022140014224000, 0x0044280028440200, 0x0008500050080400, 0x0010200020100800, 0x0020400040201000,
    0x0002000204081000, 0x0004000408102000, 0x000A000A10204000, 0x0014001422400000, 0x0028002844020000, 0x0050005008040200, 0x0020002010080400, 0x0040004020100800,
    0x0000020408102000, 0x0000040810204000, 0x00000A1020400000, 0x0000142240000000, 0x0000284402000000, 0x0000500804020000, 0x0000201008040200, 0x0000402010080400,
    0x0002040810204000, 0x0004081020400000, 0x000A102040000000, 0x0014224000000000, 0x0028440200000000, 0x0050080402000000, 0x0020100804020000, 0x0040201008040200,
];
pub const ROOK_MASK_LUT: [Bitboard; 64] =
[
    0x000101010101017E, 0x000202020202027C, 0x000404040404047A, 0x0008080808080876, 0x001010101010106E, 0x002020202020205E, 0x004040404040403E, 0x008080808080807E,
    0x0001010101017E00, 0x0002020202027C00, 0x0004040404047A00, 0x0008080808087600, 0x0010101010106E00, 0x0020202020205E00, 0x0040404040403E00, 0x0080808080807E00,
    0x00010101017E0100, 0x00020202027C0200, 0x00040404047A0400, 0x0008080808760800, 0x00101010106E1000, 0x00202020205E2000, 0x00404040403E4000, 0x00808080807E8000,
    0x000101017E010100, 0x000202027C020200, 0x000404047A040400, 0x0008080876080800, 0x001010106E101000, 0x002020205E202000, 0x004040403E404000, 0x008080807E808000,
    0x0001017E01010100, 0x0002027C02020200, 0x0004047A04040400, 0x0008087608080800, 0x0010106E10101000, 0x0020205E20202000, 0x0040403E40404000, 0x0080807E80808000,
    0x00017E0101010100, 0x00027C0202020200, 0x00047A0404040400, 0x0008760808080800, 0x00106E1010101000, 0x00205E2020202000, 0x00403E4040404000, 0x00807E8080808000,
    0x007E010101010100, 0x007C020202020200, 0x007A040404040400, 0x0076080808080800, 0x006E101010101000, 0x005E202020202000, 0x003E404040404000, 0x007E808080808000,
    0x7E01010101010100, 0x7C02020202020200, 0x7A04040404040400, 0x7608080808080800, 0x6E10101010101000, 0x5E20202020202000, 0x3E40404040404000, 0x7E80808080808000,
];
pub const KING_MASK_LUT: [Bitboard; 64] =
[
    0x0000000000000302, 0x0000000000000704, 0x0000000000000E0A, 0x0000000000001C14, 0x0000000000003828, 0x0000000000007050, 0x0000000000006020, 0x000000000001C140,
    0x0000000000030283, 0x0000000000070406, 0x00000000000E0A0E, 0x00000000001C141C, 0x0000000000382838, 0x0000000000705070, 0x00000000006020E0, 0x0000000001C140C0,
    0x0000000003028380, 0x0000000007040600, 0x000000000E0A0E00, 0x000000001C141C00, 0x0000000038283800, 0x0000000070507000, 0x000000006020E000, 0x00000001C140C000,
    0x0000000302838000, 0x0000000704060000, 0x0000000E0A0E0000, 0x0000001C141C0000, 0x0000003828380000, 0x0000007050700000, 0x0000006020E00000, 0x000001C140C00000,
    0x0000030283800000, 0x0000070406000000, 0x00000E0A0E000000, 0x00001C141C000000, 0x0000382838000000, 0x0000705070000000, 0x00006020E0000000, 0x0001C140C0000000,
    0x0003028380000000, 0x0007040600000000, 0x000E0A0E00000000, 0x001C141C00000000, 0x0038283800000000, 0x0070507000000000, 0x006020E000000000, 0x01C140C000000000,
    0x0302838000000000, 0x0704060000000000, 0x0E0A0E0000000000, 0x1C141C0000000000, 0x3828380000000000, 0x7050700000000000, 0x6020E00000000000, 0xC140C00000000000,
    0x0283800000000000, 0x0406000000000000, 0x0A0E000000000000, 0x141C000000000000, 0x2838000000000000, 0x5070000000000000, 0x20E0000000000000, 0x40C0000000000000,
];

/// Serial implementation of _pext_u64. Only used if BMI2 is not available.
/**
    Extract bits from unsigned 64-bit integer `a` at the corresponding bit
    locations specified by `mask` to contiguous low bits in `dst`. The remaining
    upper bits in `dst` are set to zero.
*/
fn pext_u64(a: u64, mask: u64) -> u64
{
    let mut dst = 0;
    let (mut k, mut m) = (1, mask);

    while m != 0
    {
        if (a & m & m.wrapping_neg()) != 0
        {
            dst |= k;
        }

        k += k;
        m &= m - 1;
    }

    dst
}

/// Serial implementation of _pdep_u64. Only used if BMI2 is not available.
/**
    Deposit contiguous low bits from unsigned 64-bit integer `a` to `dst` at the
    corresponding bit locations specified by `mask`. All other bits in `dst` are
    set to zero.
*/
fn pdep_u64(a: u64, mask: u64) -> u64
{
    let mut dst = 0;
    let (mut k, mut m) = (1, mask);

    while m != 0
    {
        if a & k != 0
        {
            dst |= m & m.wrapping_neg();
        }

        k += k;
        m &= m.saturating_sub(1);
    }

    dst
}

/// Generic attack lookup table generator.
macro_rules! create_attack_lut {
    ($lut:tt) => {
        {
            let mut table: Vec<Bitboard> = Vec::new();
            let mut offset: [usize; 64] = [0; 64];

            for (i, mask) in $lut.iter().enumerate()
            {
                offset[i] = table.len();
                table.push(*mask);

                let mut occ = *mask;

                for _ in 0 .. (1 << mask.count_ones()) - 1
                {
                    occ = occ.saturating_sub(1);
                    occ &= mask;
                    table.push(occ);
                }
            }

            return (table, offset)
        }
    };
}

/// Generates all legal knight attacks with an offset to speed up attack lookup.
pub fn create_knight_attack_lut(mask_lut: &[Bitboard]) -> (Vec<Bitboard>, [usize; 64]) { create_attack_lut!(mask_lut) }

/// Generates all legal bishop attacks with an offset to speed up attack lookup.
pub fn create_bishop_attack_lut(mask_lut: &[Bitboard]) -> (Vec<Bitboard>, [usize; 64]) { create_attack_lut!(mask_lut) }

/// Generates all legal rook attacks with an offset to speed up attack lookup.
pub fn create_rook_attack_lut(mask_lut: &[Bitboard]) -> (Vec<Bitboard>, [usize; 64]) { create_attack_lut!(mask_lut) }

/// Generates all legal king attacks with an offset to speed up attack lookup.
pub fn create_king_attack_lut(mask_lut: &[Bitboard]) -> (Vec<Bitboard>, [usize; 64]) { create_attack_lut!(mask_lut) }

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
    let mut table: [Bitboard; 64] = [0; 64];

    for (i, mask) in table.iter_mut().enumerate()
    {
        let (rank, file) = (i / 8, i % 8);

        // Create positive and negative ranges for current rank and file.
        let rf_range_pos = (rank + 1 ..= 6, file + 1 ..= 6);
        let rf_range_neg = ((1 ..= rank.saturating_sub(1)).rev(), (1 ..= file.saturating_sub(1)).rev());

        // Zip the ranges together to prepare them for chaining
        let rf_pos = rf_range_pos.0.clone().zip(rf_range_pos.1.clone());
        let rf_neg = rf_range_neg.0.clone().zip(rf_range_neg.1.clone());
        let r_pos_f_neg = rf_range_pos.0.zip(rf_range_neg.1);
        let r_neg_f_pos = rf_range_neg.0.zip(rf_range_pos.1);

        for (r, f) in rf_pos.chain(rf_neg).chain(r_pos_f_neg).chain(r_neg_f_pos)
        {
            *mask |= 1 << (f + (r * 8));
        }
    }

    table
}

/// Generates all legal rook moves.
pub fn create_rook_mask_lut() -> [Bitboard; 64]
{
    let mut table: [Bitboard; 64] = [0; 64];

    for (i, mask) in table.iter_mut().enumerate()
    {
        let (rank, file) = (i / 8, i % 8);

        for r in (rank + 1 ..= 6).chain(1 ..= rank.saturating_sub(1))
        {
            *mask |= 1 << (file + (r * 8));
        }

        for f in (file + 1 ..= 6).chain(1 ..= file.saturating_sub(1))
        {
            *mask |= 1 << (f + (rank * 8));
        }
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

        *square = n | s | e | w | ne | sw | nw | se;
    }

    table
}
