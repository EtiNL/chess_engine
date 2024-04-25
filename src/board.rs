pub mod pieces;
pub mod fen;

pub mod board{
  pub struct Board {
    white_pieces: Vect<Piece>,
    black_pieces: Vect<Piece>,
    possible_moves: Vect<(&Piece,u64)>,
    occupancy_whites: u64,
    occupancy_blacks: u64,
    occupancy_all: u64,
    fen: Fen,
  }

  pub impl Board {
    fn possible_move_generation(&mut self, color: Color) {
      let possible_moves = match color {
        Color::White => {
          let mut possible_whites_moves: Vect<(&Piece,u64)> = Vec::new();
          for piece_ref in &self.white_pieces{
            let mut piece_possible_move: u64 = 0;
            match piece_ref.piece_type {
              Piece_type::King => {}
            }
          }
        },
        Color::Black => generate_moves_whites(),
      };
    }
  }
}
