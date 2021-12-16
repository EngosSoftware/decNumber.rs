use crate::dec_dpd::BIN2DPD;
use crate::dec_number_local::QUAD_ZERO;
use std::fmt::{Debug, Formatter};

// ----------------------------------------------------------------
// Parameters for DecQuads
// ----------------------------------------------------------------

/// Length.
pub const DECQUAD_BYTES: usize = 16;
/// Maximum precision (digits).
pub const DECQUAD_PMAX: i32 = 34;
/// Minimum adjusted exponent.
pub const DECQUAD_EMIN: i32 = -6143;
/// Maximum adjusted exponent.
pub const DECQUAD_EMAX: i32 = 6144;
/// Maximum exponent digits.
pub const DECQUAD_EMAX_D: i32 = 4;
/// Bias for the exponent.
pub const DECQUAD_BIAS: i32 = 6176;
/// Maximum string length, +1.
pub const DECQUAD_STRING: i32 = 43;
/// Exponent continuation length.
pub const DECQUAD_ECON_L: i32 = 12;
/// Count of declets.
pub const DECQUAD_DECLETS: i32 = 11;
/// Highest biased exponent (Elimit - 1).
pub const DECQUAD_EHIGH: i32 = DECQUAD_EMAX + DECQUAD_BIAS - (DECQUAD_PMAX - 1);

// ----------------------------------------------------------------
// Shared constants
// ----------------------------------------------------------------

// Sign and special values.
// Top 32-bits are used.
// Last two bits are don't-care for Infinity on input.
// Last bit don't-care for NaNs.

/// 1 00000 00 Sign.
pub const DECFLOAT_SIGN: u32 = 0x80000000;
/// 0 11111 00 NaN generic.
pub const DECFLOAT_NAN: u32 = 0x7c000000;
/// 0 11111 00 qNaN.
pub const DECFLOAT_QNAN: u32 = 0x7c000000;
/// 0 11111 10 sNaN.
pub const DECFLOAT_SNAN: u32 = 0x7e000000;
/// 0 11110 00 Infinity.
pub const DECFLOAT_INF: u32 = 0x78000000;
/// Minimum special value, specials are all >= [DECFLOAT_MIN_SP].
pub const DECFLOAT_MIN_SP: u32 = 0x78000000;

// ----------------------------------------------------------------
// Sign nibble constants.
// ----------------------------------------------------------------

/// Alternate plus nibble.
#[cfg(feature = "dec-p-plus-alt")]
pub const DECPPLUSALT: u32 = 0x0A;
/// Alternate minus nibble.
#[cfg(feature = "dec-p-plus-alt")]
pub const DECPMINUSALT: u32 = 0x0B;
/// Preferred plus nibble.
#[cfg(feature = "dec-p-plus-alt")]
pub const DECPPLUS: u32 = 0x0C;
/// Preferred minus nibble.
#[cfg(feature = "dec-p-plus-alt")]
pub const DECPMINUS: u32 = 0x0D;
/// Alternate plus nibble.
#[cfg(feature = "dec-p-plus-alt")]
pub const DECPPLUSALT2: u32 = 0x0E;
/// Alternate plus nibble (unsigned).
#[cfg(feature = "dec-p-plus-alt")]
pub const DECPUNSIGNED: u32 = 0x0F;

/// The DecQuad decimal 128-bit type.
#[repr(C)]
pub union DecQuad {
  pub bytes: [u8; 16],
  pub shorts: [u16; 8],
  pub words: [u32; 4],
  pub longs: [u64; 2],
  pub quads: [u128; 1],
}

impl Debug for DecQuad {
  /// Converts [DecQuad] to a string of bytes in hex.
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      (0..16)
        .map(|i| format!(" {:02X}", unsafe { self.bytes[15 - i] }))
        .collect::<String>()
        .trim_start()
    )
  }
}

impl Default for DecQuad {
  /// Initializes [DecQuad] with zeros.
  fn default() -> Self {
    Self { bytes: [0; 16] }
  }
}

/// Initialize a [DecQuad] from [i32].
///
/// `result` gets the converted [i32] value.
/// `n` is the [i32] value to convert.
///
/// The result is exact.
/// No errors or exceptions are possible.
///
pub fn dec_quad_from_i32(result: &mut DecQuad, n: i32) -> &DecQuad {
  let mut u = n as u32;
  let mut encode: u32;
  unsafe {
    result.words[3] = QUAD_ZERO;
    result.words[2] = 0;
    result.words[1] = 0;
  }
  if n < 0 {
    u = (!u) + 1;
    unsafe {
      result.words[3] |= DECFLOAT_SIGN;
    }
  }
  encode = BIN2DPD[(u % 1000) as usize] as u32;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u32) << 10;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u32) << 20;
  u /= 1000;
  encode |= u << 30;
  unsafe {
    result.words[0] = encode;
  }
  result
}

/// Initialize a [DecQuad] from [u32].
///
/// `result` gets the converted [u32] value.
/// `n` is the [u32] value to convert.
///
/// The result is exact.
/// No errors or exceptions are possible.
///
pub fn dec_quad_from_u32(result: &mut DecQuad, mut u: u32) -> &DecQuad {
  let mut encode: u32;
  unsafe {
    result.words[3] = QUAD_ZERO;
    result.words[2] = 0;
    result.words[1] = 0;
  }
  encode = BIN2DPD[(u % 1000) as usize] as u32;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u32) << 10;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u32) << 20;
  u /= 1000;
  encode |= u << 30;
  unsafe {
    result.words[0] = encode;
    result.words[1] |= u >> 2;
  }
  result
}

/// Initialize a [DecQuad] from [i64].
///
/// `result` gets the converted [i64] value.
/// `n` is the [i64] value to convert.
///
/// The result is exact.
/// No errors or exceptions are possible.
///
pub fn dec_quad_from_i64(result: &mut DecQuad, n: i64) -> &DecQuad {
  let mut u = n as u64;
  let mut encode: u64;
  unsafe {
    result.words[3] = QUAD_ZERO;
    result.words[2] = 0;
    result.words[1] = 0;
  }
  if n < 0 {
    u = (!u) + 1;
    unsafe {
      result.words[3] |= DECFLOAT_SIGN;
    }
  }
  encode = BIN2DPD[(u % 1000) as usize] as u64;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 10;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 20;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 30;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 40;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 50;
  u /= 1000;
  encode |= u << 60;
  unsafe {
    result.longs[0] = encode;
  }
  result
}

/// Initialize a [DecQuad] from [u64].
///
/// `result` gets the converted [u64] value.
/// `n` is the [u64] value to convert.
///
/// The result is exact.
/// No errors or exceptions are possible.
///
pub fn dec_quad_from_u64(result: &mut DecQuad, mut u: u64) -> &DecQuad {
  let mut encode: u64;
  unsafe {
    result.words[3] = QUAD_ZERO;
    result.words[2] = 0;
    result.words[1] = 0;
  }
  encode = BIN2DPD[(u % 1000) as usize] as u64;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 10;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 20;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 30;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 40;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 50;
  u /= 1000;
  encode |= (BIN2DPD[(u % 1000) as usize] as u64) << 60;
  unsafe {
    result.longs[0] = encode;
    result.longs[1] |= u >> 4;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem::size_of;

  macro_rules! assert_eq_dec_quad {
    ($expected:expr, $actual:expr) => {
      assert_eq!($expected, format!("{:?}", $actual))
    };
  }

  #[test]
  fn test_dec_quad_default() {
    let dec_quad = DecQuad::default();
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], unsafe {
      dec_quad.bytes
    });
  }

  #[test]
  fn test_dec_quad_union_size() {
    assert_eq!(128, 8 * size_of::<DecQuad>());
  }

  #[test]
  fn test_dec_quad_from_i32() {
    let mut dec_quad = DecQuad::default();
    dec_quad_from_i32(&mut dec_quad, i32::MIN);
    assert_eq_dec_quad!(
      "[A2 08 00 00 00 00 00 00 00 00 00 00 8C 78 AF 48]",
      dec_quad
    );
    dec_quad_from_i32(&mut dec_quad, -1);
    assert_eq_dec_quad!(
      "[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]",
      dec_quad
    );
    dec_quad_from_i32(&mut dec_quad, -1_000);
    assert_eq_dec_quad!(
      "[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]",
      dec_quad
    );
    dec_quad_from_i32(&mut dec_quad, -1_000_000);
    assert_eq_dec_quad!(
      "[A2 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]",
      dec_quad
    );
    dec_quad_from_i32(&mut dec_quad, 0);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]",
      dec_quad
    );
    dec_quad_from_i32(&mut dec_quad, 1);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]",
      dec_quad
    );
    dec_quad_from_i32(&mut dec_quad, 1_000);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]",
      dec_quad
    );
    dec_quad_from_i32(&mut dec_quad, 1_000_000);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]",
      dec_quad
    );
    dec_quad_from_i32(&mut dec_quad, i32::MAX);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 8C 78 AF 47]",
      dec_quad
    );
  }

  #[test]
  fn test_dec_quad_from_u32() {
    let mut dec_quad = DecQuad::default();
    dec_quad_from_u32(&mut dec_quad, 0);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]",
      dec_quad
    );
    dec_quad_from_u32(&mut dec_quad, 1);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]",
      dec_quad
    );
    dec_quad_from_u32(&mut dec_quad, 1_000);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]",
      dec_quad
    );
    dec_quad_from_u32(&mut dec_quad, 1_000_000);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]",
      dec_quad
    );
    dec_quad_from_u32(&mut dec_quad, i32::MAX as u32);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 8C 78 AF 47]",
      dec_quad
    );
    dec_quad_from_u32(&mut dec_quad, u32::MAX);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 01 15 AF B5 5B]",
      dec_quad
    );
  }

  #[test]
  fn test_dec_quad_from_i64() {
    let mut dec_quad = DecQuad::default();
    dec_quad_from_i64(&mut dec_quad, i64::MIN);
    assert_eq_dec_quad!(
      "[A2 08 00 00 00 00 00 00 94 8D F2 0D A5 CF D4 2E]",
      dec_quad
    );
    dec_quad_from_i64(&mut dec_quad, 0);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]",
      dec_quad
    );
    dec_quad_from_i64(&mut dec_quad, i64::MAX);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 94 8D F2 0D A5 CF D7 0D]",
      dec_quad
    );
  }

  #[test]
  fn test_dec_quad_from_u64() {
    let mut dec_quad = DecQuad::default();
    dec_quad_from_u64(&mut dec_quad, 0);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]",
      dec_quad
    );
    dec_quad_from_u64(&mut dec_quad, u64::MAX / 3);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 63 22 9C C6 D3 6A 5D 05]",
      dec_quad
    );
    dec_quad_from_u64(&mut dec_quad, 1_844_674_407_370_955_069);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 00 19 33 74 81 DF 0B 74 69]",
      dec_quad
    );
    dec_quad_from_u64(&mut dec_quad, 18_446_744_073_709_550_610);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 01 89 1B C4 1C F8 9B 43 10]",
      dec_quad
    );
    dec_quad_from_u64(&mut dec_quad, u64::MAX);
    assert_eq_dec_quad!(
      "[22 08 00 00 00 00 00 01 89 1B C4 1C F8 9B 47 15]",
      dec_quad
    );
  }
}
