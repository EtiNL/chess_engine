pub mod fen {
  struct Fen {
    piece_placement: String,
    active_color: String,
    castling_rights: String,
    possible_en_passant_target: String,
    half_move_clock: u8,
    full_move_number: u8,
  }
}
