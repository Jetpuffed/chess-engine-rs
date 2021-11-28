//! <Module description goes here>

pub mod movegen;
pub mod tables;

/// Value that represents a set of 64 individual bits.
pub type Bitboard = u64;

/**
    Little-Endian Rank-File Mapping.

    Each variant contains:
    - `id:` Indicates which square the variant represents.
    - `n_lut:` Raw pointer to knight attack lookup table.
    - `b_lut:` Raw pointer to bishop attack lookup table.
    - `r_lut:` Raw pointer to rook attack lookup table.
    - `q_lut:` Raw pointer to an intersection between bishop and rook attack lookup table.
    - `k_lut:` Raw pointer to king attack lookup table.
*/ 
pub enum Map
{
    A1 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    B1 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    C1 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    D1 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    E1 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    F1 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    G1 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    H1 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    A2 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    B2 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    C2 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    D2 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    E2 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    F2 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    G2 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    H2 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    A3 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    B3 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    C3 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    D3 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    E3 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    F3 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    G3 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    H3 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    A4 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    B4 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    C4 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    D4 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    E4 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    F4 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    G4 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    H4 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    A5 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    B5 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    C5 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    D5 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    E5 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    F5 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    G5 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    H5 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    A6 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    B6 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    C6 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    D6 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    E6 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    F6 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    G6 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    H6 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    A7 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    B7 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    C7 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    D7 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    E7 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    F7 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    G7 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    H7 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    A8 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    B8 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    C8 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    D8 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    E8 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    F8 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    G8 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
    H8 { id: usize, n_lut: *const [Bitboard], b_lut: *const [Bitboard], r_lut: *const [Bitboard], q_lut: *const [Bitboard], k_lut: *const [Bitboard] },
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
