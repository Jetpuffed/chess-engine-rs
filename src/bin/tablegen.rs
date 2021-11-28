use chess_engine_rs::bitboard::tables::
{
    create_bishop_mask_lut,
    create_knight_mask_lut,
    create_rook_mask_lut,
    create_king_mask_lut,
};

fn main()
{
    // Create a new buffer for stdin.
    let mut buf = String::new();

    println!("Ready to generate knight mask lookup table. Press any key to continue.");
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("pub const KNIGHT_MASK_LUT: [Bitboard; 64] = {:#018X?};\n", create_knight_mask_lut());

    println!("Ready to generate bishop mask lookup table. Press any key to continue.");
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("pub const BISHOP_MASK_LUT: [Bitboard; 64] = {:#018X?};\n", create_bishop_mask_lut());

    println!("Ready to generate rook mask lookup table. Press any key to continue.");
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("pub const ROOK_MASK_LUT: [Bitboard; 64] = {:#018X?};\n", create_rook_mask_lut());

    println!("Ready to generate king mask lookup table. Press any key to continue.");
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("pub const KING_MASK_LUT: [Bitboard; 64] = {:#018X?};\n", create_king_mask_lut());
}