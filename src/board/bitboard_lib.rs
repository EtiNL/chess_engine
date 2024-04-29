pub mod bitboard_lib {

  fn flip_vertical(bitboard: u64) -> u64 {
    let mut x = bitboard;
    let k1: u64 = 0x00ff00ff00ff00ff;
    let k2: u64 = 0x0000ffff0000ffff;
    x = ((x >>  8) & k1) | ((x & k1) <<  8);
    x = ((x >> 16) & k2) | ((x & k2) << 16);
    x = (x >> 32) | (x << 32);
    x
  }

  fn flip_diag_a1_h8(bitboard: u64) -> u64 {
    let mut x = bitboard;
    let k1: u64 = 0x5500550055005500;
    let k2: u64 = 0x3333000033330000;
    let k4: u64 = 0x0f0f0f0f00000000;
    let mut t  = k4 & (x ^ (x << 28));
    x ^= t ^ (t >> 28) ;
    t  = k2 & (x ^ (x << 14));
    x ^= t ^ (t >> 14) ;
    t  = k1 & (x ^ (x <<  7));
    x ^= t ^ (t >>  7) ;
    x
  }

  pub fn rotation_90_degree_clockwise(bitboard: u64) -> u64 {
    flip_vertical(flip_diag_a1_h8(bitboard))
  }

  pub fn rotation_90_degree_anticlockwise(bitboard: u64) -> u64 {
    flip_diag_a1_h8(flip_vertical(bitboard))
  }
  pub static a_file: u64 = 0x0101010101010101;
  pub static h_file: u64 = 0x0808080808080808;
  pub static first_rank: u64 = 0xff;
  pub static last_rank: u64 = 0xff00000000000000;
  pub static rank_2: u64 = 0x000000000000ff00;
  pub static rank_7: u64 = 0x000000000000ff00;
  pub static diagonals: [u64; 15] = [0x80, // h1_h1
                                     0x8040, // g1_h2
                                     0x804020, // f1_h3
                                     0x80402010, // e1_h4
                                     0x8040201008, // d1_h5
                                     0x804020100804, // c1_h6
                                     0x80402010080402, // b1_h7
                                     0x8040201008040201, // a1_h8
                                     0x4020100804020100, // a2_g8
                                     0x2010080402010000, // a3_f8
                                     0x1008040201000000, // a4_e8
                                     0x0804020100000000, // a5_d8
                                     0x0402010000000000, // a6_c8
                                     0x0201000000000000, // a7_b8
                                     0x0100000000000000  // a8_a8
                                    ];
  pub static antidiagonals: [u64; 15] = [0x01, //a1_a1
                                         0x0102, // a2_b1
                                         0x010204, // a3_c1
                                         0x01020408, // a4_d1
                                         0x0102040810, // a5_e1
                                         0x010204081020, // a6_f1
                                         0x01020408102040, // a7_g1
                                         0x0102040810204080, // a8_h1
                                         0x0204081020408000, // b8_h2
                                         0x0408102040800000, // c8_h3
                                         0x0810204080000000, // d8_h4
                                         0x1020408000000000, // e8_h5
                                         0x2040800000000000, // f8_h6
                                         0x4080000000000000, // g8_h7
                                         0x8000000000000000  // h8_h8
                                        ];
}
