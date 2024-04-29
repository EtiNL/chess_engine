pub mod bitboard_lib;

pub mod pieces {
  enum Color {
    Black,
    White,
  }

  enum Piece_type {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
  }

  pub struct Piece {
    piece_type: Piece_type,
    bitboard: u64,
  }
  pub impl Piece {
    fn pseudo_possible_move(&self, occupancy_all: u64, occupancy_opposite_color: u64) -> u64 {
      match self.piece_type {
        Piece_type::King => {pseudo_possible_king_move(self.bitboard, occupancy_color)},
        Piece_type::Queen => {pseudo_possible_queen_move(self.bitboard, occupancy_color, occupancy_all)},
        Piece_type::Rook => {pseudo_possible_rook_move(self.bitboard, occupancy_color, occupancy_all)},
        Piece_type::Bishop => {pseudo_possible_bishop_move(self.bitboard, occupancy_color, occupancy_all)},
        Piece_type::Knight => {pseudo_possible_knight_move(self.bitboard, occupancy_color)},
        Piece_type::Pawn(color) => {pseudo_possible_pawn_move(self.bitboard, occupancy_color, occupancy_opposite_color, color)},
      }
    }
  }

  fn pseudo_possible_king_move(king: u64, occupancy_color: u64) -> u64 {
      let mut moves: u64 = 0;
      let rank = king.trailing_zeros() / 8;
      let file = king.trailing_zeros() % 8;

      // Moves in each direction (castling move will be handled downstream)
      if file > 0 { moves |= king >> 1; }               // West
      if file < 7 { moves |= king << 1; }               // East
      if rank > 0 { moves |= king >> 8; }               // North
      if rank < 7 { moves |= king << 8; }               // South
      if file > 0 && rank > 0 { moves |= king >> 9; }   // Northwest
      if file < 7 && rank > 0 { moves |= king >> 7; }   // Northeast
      if file > 0 && rank < 7 { moves |= king << 7; }   // Southwest
      if file < 7 && rank < 7 { moves |= king << 9; }   // Southeast

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

  fn pseudo_possible_rook_move(rooks: u64, occupancy_color: u64, occupancy: u64) -> u64 {
    let mut moves: u64 = 0;

    let mut rook_positions = rooks;
    while rook_positions != 0 {

        let rook = 1 << rook_positions.trailing_zeros(); // Position of the least significant bit
        let rank_ray_attack = (occupancy - 2*rook) ^ (occupancy.reverse_bits() - 2*(rook.reverse_bits())).reverse_bits();

        let rook_rot = bitboard_lib::rotation_90_degree_clockwise(rook);
        let occupancy_rot = bitboard_lib::rotation_90_degree_clockwise(occupancy);
        let file_ray_attack = bitboard_lib::rotation_90_degree_anticlockwise((occupancy_rot - 2*rook_rot) ^ (occupancy_rot.reverse_bits() - 2*(rook_rot.reverse_bits())).reverse_bits());

        rook_positions &= rook_positions - 1; // Clear the least significant bit
    }
    moves & !occupancy_color
  }

  fn pseudo_possible_bishop_move(bishops: u64, occupancy_color: u64, occupancy: u64) -> u64 {
    let mut moves: u64 = 0;

    let mut bishop_positions = bishops;

    while bishop_positions != 0 {
      let bishop_pos = bishop_positions.trailing_zeros();
      let bishop = 1 << bishop_pos;
      let file = bishop_pos%8;
      let rank = bishop_pos/8;

      let diagonal_mask = bitboard_lib::diagonals[7 + rank - file];
      let diagonal_ray_attack = ((occupancy & diagonal_mask - 2*bishop) ^
                                ((occupancy & diagonal_mask).reverse_bits() - 2*(bishop.reverse_bits())))
                                & diagonal_mask;

      let antidiagonal_mask = bitboard_lib::antidiagonals[rank + file];
      let antidiagonal_ray_attack = ((occupancy & antidiagonal_mask - 2*bishop) ^
                                    ((occupancy & antidiagonal_mask).reverse_bits() - 2*(bishop.reverse_bits())))
                                    & antidiagonal_mask;

      moves |= (diagonal_ray_attack | antidiagonal_ray_attack);
      bishop_positions &= bishop_positions - 1;
    }
    moves & !occupancy_color
  }

  fn pseudo_possible_queen_move(queens: u64, occupancy_color: u64, occupancy: u64) -> u64 {
    let mut moves: u64 = 0;

    let mut queen_positions = queens;

    while queen_positions != 0 {
      let queen_pos = queen_positions.trailing_zeros();
      let queen = 1 << queen_pos;
      let file = queen_pos%8;
      let rank = queen_pos/8;

      let diagonal_mask = bitboard_lib::diagonals[7 + rank - file];
      let diagonal_ray_attack = ((occupancy & diagonal_mask - 2*queen) ^
                                ((occupancy & diagonal_mask).reverse_bits() - 2*(queen.reverse_bits())))
                                & diagonal_mask;

      let antidiagonal_mask = bitboard_lib::antidiagonals[rank + file];
      let antidiagonal_ray_attack = ((occupancy & antidiagonal_mask - 2*queen) ^
                                    ((occupancy & antidiagonal_mask).reverse_bits() - 2*(queen.reverse_bits())))
                                    & antidiagonal_mask;

      let rank_ray_attack = (occupancy - 2*queen) ^ (occupancy.reverse_bits() - 2*(queen.reverse_bits())).reverse_bits();

      let queen_rot = bitboard_lib::rotation_90_degree_clockwise(queen);
      let occupancy_rot = bitboard_lib::rotation_90_degree_clockwise(occupancy);
      let file_ray_attack = bitboard_lib::rotation_90_degree_anticlockwise((occupancy_rot - 2*queen_rot) ^ (occupancy_rot.reverse_bits() - 2*(queen_rot.reverse_bits())).reverse_bits());

      moves |= (diagonal_ray_attack | antidiagonal_ray_attack | rank_ray_attack | file_ray_attack);
      queen_positions &= queen_positions - 1;
    }
    moves & !occupancy_color
  }

  fn pseudo_possible_pawn_move(pawns: u64, occupancy_color: u64, occupancy_opposite_color: u64, color: Color) -> u64 {
    let mut moves: u64 = 0; // The en-passant move and promotion will be handled downstream
    match color {
      Color::White => {moves |= (pawns << 8) & !occupancy_color & !occupancy_opposite_color; // moving one square forward
                       moves |= ((pawns & bitboard_lib::rank_2) << 16) & !occupancy_color & !occupancy_opposite_color; // moving two square forward
                       moves |= (pawns << 7) & !bitboard_lib::h_file & occupancy_opposite_color; // attacking on the left
                       moves |= (pawns << 9) & !bitboard_lib::a_file & occupancy_opposite_color; // attacking on the right
                      },
      Color::Black => {moves |= (pawns >> 8) & !occupancy_color & !occupancy_opposite_color; // moving one square forward
                       moves |= ((pawns & bitboard_lib::rank_7) << 16) & !occupancy_color & !occupancy_opposite_color; // moving two square forward
                       moves |= (pawns >> 7) & !bitboard_lib::h_file & occupancy_opposite_color; // attacking on the left
                       moves |= (pawns >> 9) & !bitboard_lib::a_file & occupancy_opposite_color; // attacking on the right
                      },
    }
    moves
  }
}
