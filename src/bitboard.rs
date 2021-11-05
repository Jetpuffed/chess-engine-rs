//! <Module description goes here>

/// Value that represents a set of 64 individual bits.
type Bitboard = u64;

/// Value that represents a location in a bitboard mapping.
type Square = Bitmap;

/// Little-Endian Rank-File Mapping.
#[repr(C)]
enum Bitmap
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
struct Board
{
    pawn: Bitboard,
    knight: Bitboard,
    bishop: Bitboard,
    rook: Bitboard,
    queen: Bitboard,
    king: Bitboard,
}

/// Supertrait containing all operations a subtrait must implement in order to be a valid piece.
trait Piece
{
    const ID: u64;
    fn make_move(&self);
}

/// Subtrait of `Piece` containing all operations unique to a pawn.
trait Pawn : Piece
{
    fn single_push(&self);
    fn double_push(&self);
    fn en_passant(&self);
    fn promote(&self);
}

/// Subtrait of `Piece` containing all operations unique to a knight.
trait Knight : Piece {}

/// Subtrait of `Piece` containing all operations unique to a bishop.
trait Bishop : Piece  {}

/// Subtrait of `Piece` containing all operations unique to a rook.
trait Rook : Piece {}

/// Subtrait of `Piece` containing all operations unique to a queen.
trait Queen : Piece {}

/// Subtrait of `Piece` containing all operations unique to a king.
trait King : Piece {}
