pub mod pieces {
  enum Color {
    Black,
    White,
  }

  enum Piece_type {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
  }

  pub struct Piece {
    color: Color,
    piece_type: Piece_type,
    bitboard: u64,
  }
  pub impl Piece {
    fn pseudo_possible_move(&self, occupancy_all: u64, occupancy_opposite_color: u64) -> u64 {
      match self.piece_type {
        Piece_type::King => {pseudo_possible_king_move(self.bitboard, occupancy_color)},
        Piece_type::Queen => {pseudo_possible_queen_move(self.bitboard, occupancy_color, occupancy_opposite_color)},
        Piece_type::Rook => {pseudo_possible_rook_move(self.bitboard, occupancy_color, occupancy_opposite_color)},
        Piece_type::Bishop => {pseudo_possible_bishop_move(self.bitboard, occupancy_color, occupancy_opposite_color)},
        Piece_type::Knight => {pseudo_possible_knight_move(self.bitboard, occupancy_color)},
        Piece_type::Pawn => {pseudo_possible_pawn_move(self.bitboard, occupancy_color, occupancy_opposite_color)},
      }
    }
  }

  fn pseudo_possible_king_move(king: u64, occupancy_color: u64) -> u64 {
      let mut moves: u64 = 0;
      let rank = king / 8;
      let file = king % 8;

      // Moves in each direction (castling move will be handled downstream)
      if file > 0 { moves |= 1 << (king - 1); }               // West
      if file < 7 { moves |= 1 << (king + 1); }               // East
      if rank > 0 { moves |= 1 << (king - 8); }               // North
      if rank < 7 { moves |= 1 << (king + 8); }               // South
      if file > 0 && rank > 0 { moves |= 1 << (king - 9); }   // Northwest
      if file < 7 && rank > 0 { moves |= 1 << (king - 7); }   // Northeast
      if file > 0 && rank < 7 { moves |= 1 << (king + 7); }   // Southwest
      if file < 7 && rank < 7 { moves |= 1 << (king + 9); }   // Southeast

      moves & !occupancy_color
  }


  fn pseudo_possible_knight_move(knights: u64, occupancy_color: u64) -> u64 {
    let mut moves: u64 = 0;

    let mut knight_positions = knights;
    while knight_positions != 0 {
        let knight = knight_positions.trailing_zeros(); // Position of the least significant bit
        let rank = knight / 8;
        let file = knight % 8;

        // Moves in each direction
        if file > 1 && rank < 7 { moves |= 1 << (knight + 6); }
        if file > 1 && rank > 0 { moves |= 1 << (knight - 10); }
        if file > 0 && rank > 1 { moves |= 1 << (knight - 17); }
        if file < 7 && rank > 1 { moves |= 1 << (knight - 15); }
        if file < 6 && rank > 0 { moves |= 1 << (knight - 6); }
        if file < 6 && rank < 7 { moves |= 1 << (knight + 10); }
        if file < 7 && rank < 6 { moves |= 1 << (knight + 17); }
        if file > 0 && rank < 6 { moves |= 1 << (knight + 15); }

        knight_positions &= knight_positions - 1; // Clear the least significant bit
    }
    moves & !occupancy_color
  }

  fn rotate_90_clockwise(bitboard: u64) -> u64 {
    const K1: u64 = 0x00FF00FF00FF00FF;
    const K2: u64 = 0x0000FFFF0000FFFF;
    let x = bitboard;
    let t = (x ^ (x >> 28)) & K1;
    let x = x ^ t ^ (t << 28);
    let t = (x ^ (x >> 14)) & K2;
    let x = x ^ t ^ (t << 14);
    let t = (x ^ (x >> 7)) & 0x5555555555555555;
    x ^ t ^ (t << 7)
  }

  fn rotate_90_anticlockwise(bitboard: u64) -> u64 {
    const K1: u64 = 0xAA00AA00AA00AA00;
    const K2: u64 = 0xCCCC0000CCCC0000;
    const K3: u64 = 0xF0F0F0F00F0F0F0F;
    let x = bitboard;
    let t = (x ^ (x >> 36)) & K3;
    let x = x ^ t ^ (t << 36);
    let t = (x ^ (x >> 18)) & K2;
    let x = x ^ t ^ (t << 18);
    let t = (x ^ (x >> 9)) & K1;
    x ^ t ^ (t << 9)
  }

  fn pseudo_possible_rook_move(rooks: u64, occupancy_color: u64, occupancy_opposite_color: u64) -> u64 {
    let mut moves: u64 = 0;

    let mut rook_positions = rooks;
    while rook_positions != 0 {
        let rook = rook_positions.trailing_zeros(); // Position of the least significant bit
        let rank_mask = 0xff << (rook_position & 56); // Mask for the rank of the rook: 0xff is a binary mask where the lowest 8 bits are set (1111_1111)
                                                      // rook_position & 56 computes the start of the rank for a given rook_position. Here, 56 is 0b111000
                                                      // This operation finds the start of the rank (the first square) where the rook is located.
                                                      // << shifts the 0xff mask to the left by the start of the rank, aligning the mask with the rank where the rook is positioned.
                                                      // This sets up a mask that covers only the squares of the rook's rank.
        let horizontal_moves = ((occupied & rank_mask).wrapping_sub(1 << rook_position)) ^ ((occupied & rank_mask).reverse_bits().wrapping_sub((1 << rook_position).reverse_bits())).reverse_bits();

        let rotated_rook_position = rotate_90_clockwise(1 << rook_position).trailing_zeros();
        let file_mask = rotate_90_anticlockwise(0xff << (rotated_rook_position & 56));

        moves |= rank_mask & file_mask;

        rook_positions &= rook_positions - 1; // Clear the least significant bit
    }
    moves & !occupancy_color
  }
}
