use std::io::{Error, Write};

use chess_engine_rs::bitboard::tables::
{
    create_bishop_attack_lut,
    create_bishop_mask_lut,
    create_king_attack_lut,
    create_king_mask_lut,
    create_knight_attack_lut,
    create_knight_mask_lut,
    create_rook_attack_lut,
    create_rook_mask_lut
};

fn main() -> Result<(), Error>
{
    // Create a new output file or use an already existing one, overwriting the previous output.
    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("tables.txt")?;

    // Create a new buffer to hold all formatted tables before writing to file
    let mut buf = String::new();

    /*
        Mask lookup table generators
    */

    buf.push_str(format!("pub const KNIGHT_MASK_LUT: [Bitboard; 64] = {:#018X?};\n", create_knight_mask_lut()).as_str());
    buf.push_str(format!("pub const BISHOP_MASK_LUT: [Bitboard; 64] = {:#018X?};\n", create_bishop_mask_lut()).as_str());
    buf.push_str(format!("pub const ROOK_MASK_LUT: [Bitboard; 64] = {:#018X?};\n", create_rook_mask_lut()).as_str());
    buf.push_str(format!("pub const KING_MASK_LUT: [Bitboard; 64] = {:#018X?};\n", create_king_mask_lut()).as_str());

    /*
        Attack lookup table generators
    */

    let (lut, offset) = create_knight_attack_lut();
    buf.push_str(format!("pub const KNIGHT_ATTACK_LUT: [Bitboard; {}] = {:#018X?};\n", lut.len(), lut).as_str());
    buf.push_str(format!("pub const KNIGHT_ATTACK_OFFSET: [Bitboard; 64] = {:?};\n", offset).as_str());

    let (lut, offset) = create_bishop_attack_lut();
    buf.push_str(format!("pub const BISHOP_ATTACK_LUT: [Bitboard; {}] = {:#018X?};\n", lut.len(), lut).as_str());
    buf.push_str(format!("pub const BISHOP_ATTACK_OFFSET: [Bitboard; 64] = {:?};\n", offset).as_str());

    let (lut, offset) = create_rook_attack_lut();
    buf.push_str(format!("pub const ROOK_ATTACK_LUT: [Bitboard; {}] = {:#018X?};\n", lut.len(), lut).as_str());
    buf.push_str(format!("pub const ROOK_ATTACK_OFFSET: [Bitboard; 64] = {:?};\n", offset).as_str());

    let (lut, offset) = create_king_attack_lut();
    buf.push_str(format!("pub const KING_ATTACK_LUT: [Bitboard; {}] = {:#018X?};\n", lut.len(), lut).as_str());
    buf.push_str(format!("pub const KING_ATTACK_OFFSET: [Bitboard; 64] = {:?};\n", offset).as_str());

    // Write the buffer to the output file, returning either a success value or an error.
    output_file.write_all(buf.as_bytes())
}