/// Type alias for u8.
pub type Ubyte = u8;

/// Type alias for u32.
pub type Uint = u32;

/// Type alias for i32.
pub type Int = i32;

// ----------------------------------------------------------------------------
// Top words for a zero
// ----------------------------------------------------------------------------

/// Top word for single zero.
pub const SINGLEZERO: Uint = 0x22500000;

/// Top word for double zero.
pub const DOUBLEZERO: Uint = 0x22380000;

/// Top word for quad zero.
pub const QUADZERO: Uint = 0x22080000;

///
pub const DEC_COMB_TO_EXP: [i32; 64] = [
  0, 0, 0, 0, 0, 0, 0, 0, 4096, 4096, 4096, 4096, 4096, 4096, 4096, 4096, 8192,
  8192, 8192, 8192, 8192, 8192, 8192, 8192, 0, 0, 4096, 4096, 8192, 8192,
  0x78000000, 0x7c000000, 0, 0, 0, 0, 0, 0, 0, 0, 4096, 4096, 4096, 4096, 4096,
  4096, 4096, 4096, 8192, 8192, 8192, 8192, 8192, 8192, 8192, 8192, 0, 0, 4096,
  4096, 8192, 8192, 0x78000000, 0x7c000000,
];

///
pub const DEC_COMB_TO_MSD: [usize; 64] = [
  0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
  8, 9, 8, 9, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3,
  4, 5, 6, 7, 8, 9, 8, 9, 8, 9, 0, 0,
];

// ----------------------------------------------------------------------------
// Macros to test if a certain 10 bits of a uInt or pair of uInts
// are a canonical declet (higher or lower bits are ignored).
// ----------------------------------------------------------------------------

/// Tests if declet is at offset 0 (from the right) in a [UInt].
pub fn canonical_dpd(dpd: &Uint) -> bool {
  (dpd & 0x300) == 0 || (dpd & 0x6e) != 0x6e
}

/// Tests if declet is at offset k (a multiple of 2) in a [Uint].
pub fn canonical_dpd_off(dpd: &Uint, k: usize) -> bool {
  ((dpd) & (0x300 << (k))) == 0
    || ((dpd) & ((0x6e_u32) << (k))) != ((0x6e_u32) << (k))
}

/// Tests if declet is at offset k (a multiple of 2) in a pair of [Uints](Uint),
/// the top 2 bits will always be in the more-significant [Uint].
pub fn canonical_dpd_two(hi: &Uint, lo: &Uint, k: usize) -> bool {
  ((hi) & (0x300 >> (32 - (k)))) == 0
    || ((hi) & (0x6e >> (32 - (k)))) != (0x6e >> (32 - (k)))
    || ((lo) & ((0x6e_u32) << (k))) != ((0x6e_u32) << (k))
}

/// Round an integer up to a multiple of n.
pub fn round_up(i: usize, n: usize) -> usize {
  (((i) + (n) - 1) / n) * n
}
