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

    // Create a new buffer to hold all formatted tables before writing to file.
    let mut buf = String::new();

    // Create a closure expression to fit multiple values on a single line.
    let compress_table = |lut|
    {
        let mut output = Vec::new();
        let mut fmt_buf = Vec::new();

        for bb in lut
        {
            fmt_buf.push(format!("{:#018X}", bb));

            if fmt_buf.len() == 8
            {
                output.push(fmt_buf.join(", ") + ",");
                fmt_buf.clear();
            }
        }

        if !fmt_buf.is_empty() { output.push(fmt_buf.join(", ")) }

        output
    };

    /*
        Mask lookup table generators
    */

    let lut = compress_table(create_knight_mask_lut().to_vec());
    buf.push_str("pub const KNIGHT_MASK_LUT: [Bitboard; 64] = [\n");
    for s in lut { buf.push_str(&("    ".to_owned() + &s + "\n")) }
    buf.push_str("];\n\n");

    let lut = compress_table(create_bishop_mask_lut().to_vec());
    buf.push_str("pub const BISHOP_MASK_LUT: [Bitboard; 64] = [\n");
    for s in lut { buf.push_str(&("    ".to_owned() + &s + "\n")) }
    buf.push_str("];\n\n");

    let lut = compress_table(create_rook_mask_lut().to_vec());
    buf.push_str("pub const ROOK_MASK_LUT: [Bitboard; 64] = [\n");
    for s in lut { buf.push_str(&("    ".to_owned() + &s + "\n")) }
    buf.push_str("];\n\n");

    let lut = compress_table(create_king_mask_lut().to_vec());
    buf.push_str("pub const KING_MASK_LUT: [Bitboard; 64] = [\n");
    for s in lut { buf.push_str(&("    ".to_owned() + &s + "\n")) }
    buf.push_str("];\n\n");

    /*
        Attack lookup table generators
    */

    let (lut, offset) = create_knight_attack_lut();
    buf.push_str(format!("pub const KNIGHT_ATTACK_LUT: [Bitboard; {}] = [\n", lut.len()).as_str());
    for s in compress_table(lut) { buf.push_str(&("    ".to_owned() + &s + "\n")) }
    buf.push_str("];\n\n");

    buf.push_str(format!("pub const KNIGHT_ATTACK_OFFSET: [usize; 64] = {:?};\n\n", offset).as_str());

    let (lut, offset) = create_bishop_attack_lut();
    buf.push_str(format!("pub const BISHOP_ATTACK_LUT: [Bitboard; {}] = [\n", lut.len()).as_str());
    for s in compress_table(lut) { buf.push_str(&("    ".to_owned() + &s + "\n")) }
    buf.push_str("];\n\n");

    buf.push_str(format!("pub const BISHOP_ATTACK_OFFSET: [usize; 64] = {:?};\n\n", offset).as_str());

    let (lut, offset) = create_rook_attack_lut();
    buf.push_str(format!("pub const ROOK_ATTACK_LUT: [Bitboard; {}] = [\n", lut.len()).as_str());
    for s in compress_table(lut) { buf.push_str(&("    ".to_owned() + &s + "\n")) }
    buf.push_str("];\n\n");

    buf.push_str(format!("pub const ROOK_ATTACK_OFFSET: [usize; 64] = {:?};\n\n", offset).as_str());

    let (lut, offset) = create_king_attack_lut();
    buf.push_str(format!("pub const KING_ATTACK_LUT: [Bitboard; {}] = [\n", lut.len()).as_str());
    for s in compress_table(lut) { buf.push_str(&("    ".to_owned() + &s + "\n")) }
    buf.push_str("];\n\n");

    buf.push_str(format!("pub const KING_ATTACK_OFFSET: [usize; 64] = {:?};\n\n", offset).as_str());

    // Write the buffer to the output file, returning either a success value or an error.
    output_file.write_all(buf.as_bytes())
}