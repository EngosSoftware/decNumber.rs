///
pub const DEC_COMB_TO_EXP: [i32; 64] = [
  0, 0, 0, 0, 0, 0, 0, 0, 4096, 4096, 4096, 4096, 4096, 4096, 4096, 4096, 8192, 8192, 8192, 8192,
  8192, 8192, 8192, 8192, 0, 0, 4096, 4096, 8192, 8192, 0x78000000, 0x7c000000, 0, 0, 0, 0, 0, 0,
  0, 0, 4096, 4096, 4096, 4096, 4096, 4096, 4096, 4096, 8192, 8192, 8192, 8192, 8192, 8192, 8192,
  8192, 0, 0, 4096, 4096, 8192, 8192, 0x78000000, 0x7c000000,
];

///
pub const DEC_COMB_TO_MSD: [usize; 64] = [
  0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 9, 8, 9, 0, 0,
  0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 9, 8, 9, 0, 0,
];