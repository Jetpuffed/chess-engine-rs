//! <Module description goes here>

/// Value that represents a set of 64 individual bits.
pub type Bitboard = u64;

/// Little-Endian Rank-File Mapping.
#[repr(C)]
pub enum Map
{
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

/// Dense board structure that contains bitboards for individual piece types.
pub struct Board
{
    white:  Bitboard,
    black:  Bitboard,
    pawn:   Bitboard,
    knight: Bitboard,
    bishop: Bitboard,
    rook:   Bitboard,
    queen:  Bitboard,
    king:   Bitboard,
}

impl Board
{
    /// Creates a new board with custom starting positions.
    pub fn new() -> Self { todo!() }

    /// Get the positions of all white pieces.
    pub fn get_white(&self) -> Bitboard { self.white }

    /// Get the positions of all black pieces.
    pub fn get_black(&self) -> Bitboard { self.black }

    /// Get the positions of all pawns, regardless of color.
    pub fn get_pawns(&self) -> Bitboard { self.pawn }

    /// Get the positions of all knights, regardless of color.
    pub fn get_knights(&self) -> Bitboard { self.knight }

    /// Get the positions of all bishops, regardless of color.
    pub fn get_bishops(&self) -> Bitboard { self.bishop }

    /// Get the positions of all rooks, regardless of color.
    pub fn get_rooks(&self) -> Bitboard { self.rook }

    /// Get the positions of all queens, regardless of color.
    pub fn get_queens(&self) -> Bitboard { self.queen }

    /// Get the positions of all kings, regardless of color.
    pub fn get_kings(&self) -> Bitboard { self.king }
}

impl Default for Board
{
    /// Creates a new board with the default starting positions.
    fn default() -> Self
    {
        Self
        {
            white:  0x000000000000FFFF,
            black:  0xFFFF000000000000,
            pawn:   0x00FF00000000FF00,
            knight: 0x2400000000000024,
            bishop: 0x4200000000000042,
            rook:   0x8100000000000081,
            queen:  0x0800000000000008,
            king:   0x1000000000000010,
        }
    }
}
