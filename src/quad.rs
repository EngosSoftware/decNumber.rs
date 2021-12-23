use crate::dpd::BIN2DPD;
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
pub const DEC_QUAD_BIAS: i32 = 6176;
/// Maximum string length, +1.
pub const DECQUAD_STRING: i32 = 43;
/// Exponent continuation length.
pub const DEC_QUAD_ECONL: i32 = 12;
/// Count of declets.
pub const DECQUAD_DECLETS: i32 = 11;
/// Highest biased exponent (Elimit - 1).
pub const DECQUAD_EHIGH: i32 = DECQUAD_EMAX + DEC_QUAD_BIAS - (DECQUAD_PMAX - 1);

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

///
pub const DEC_QUAD_POSITIVE_ZERO: DecQuad = DecQuad {
  bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22],
};

///
pub const DEC_QUAD_NEGATIVE_ZERO: DecQuad = DecQuad {
  bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0xA2],
};

/// The DecQuad decimal 128-bit type.
#[repr(C)]
pub union DecQuad {
  pub bytes: [u8; 16],
  pub shorts: [u16; 8],
  pub words: [u32; 4],
  pub longs: [u64; 2],
}

impl Debug for DecQuad {
  /// Converts [DecQuad] to a string containing hexadecimal bytes.
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      (0..16)
        .into_iter()
        .rev()
        .map(|i| format!(" {:02X}", unsafe { self.bytes[i] }))
        .collect::<String>()
        .trim_start()
    )
  }
}

impl Default for DecQuad {
  /// Initializes [DecQuad] with zeros.
  fn default() -> Self {
    DEC_QUAD_POSITIVE_ZERO
  }
}

impl From<i8> for DecQuad {
  /// Returns a [DecQuad] initialized from [i8] value.
  ///
  /// Result is exact, without any errors or exceptions.
  ///
  fn from(n: i8) -> Self {
    let mut unsigned = n as u8;
    let mut dq = if n < 0 {
      unsigned = (!unsigned) + 1;
      DEC_QUAD_NEGATIVE_ZERO
    } else {
      DEC_QUAD_POSITIVE_ZERO
    };
    unsafe {
      dq.shorts[0] = BIN2DPD[unsigned as usize];
    }
    dq
  }
}

impl From<u8> for DecQuad {
  /// Returns a [DecQuad] initialized from [u8] value.
  ///
  /// Result is exact, without any errors or exceptions.
  ///
  fn from(n: u8) -> Self {
    let mut dq = DEC_QUAD_POSITIVE_ZERO;
    unsafe {
      dq.shorts[0] = BIN2DPD[n as usize];
    }
    dq
  }
}

impl From<i16> for DecQuad {
  /// Returns a [DecQuad] initialized from [i16] value.
  ///
  /// Result is exact, without any errors or exceptions.
  ///
  fn from(n: i16) -> Self {
    let mut unsigned = n as u16;
    let mut dq = if n < 0 {
      unsigned = (!unsigned) + 1;
      DEC_QUAD_NEGATIVE_ZERO
    } else {
      DEC_QUAD_POSITIVE_ZERO
    };
    let mut encoded = BIN2DPD[(unsigned % 1000) as usize] as u32;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u32) << 10;
    unsigned /= 1000;
    encoded |= (unsigned as u32) << 20;
    unsafe {
      dq.words[0] = encoded;
    }
    dq
  }
}

impl From<u16> for DecQuad {
  /// Returns a [DecQuad] initialized from [u16] value.
  ///
  /// Result is exact, without any errors or exceptions.
  ///
  fn from(n: u16) -> Self {
    let mut dq = DEC_QUAD_POSITIVE_ZERO;
    let mut unsigned = n;
    let mut encoded = BIN2DPD[(unsigned % 1000) as usize] as u32;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u32) << 10;
    unsigned /= 1000;
    encoded |= (unsigned as u32) << 20;
    unsafe {
      dq.words[0] = encoded;
    }
    dq
  }
}

impl From<i32> for DecQuad {
  /// Returns a [DecQuad] initialized from [i32] value.
  ///
  /// Result is exact, without any errors or exceptions.
  ///
  fn from(n: i32) -> Self {
    let mut unsigned = n as u32;
    let mut dq = if n < 0 {
      unsigned = (!unsigned) + 1;
      DEC_QUAD_NEGATIVE_ZERO
    } else {
      DEC_QUAD_POSITIVE_ZERO
    };
    let mut encoded = BIN2DPD[(unsigned % 1000) as usize] as u64;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 10;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 20;
    unsigned /= 1000;
    encoded |= (unsigned as u64) << 30;
    unsafe {
      dq.longs[0] = encoded;
    }
    dq
  }
}

impl From<u32> for DecQuad {
  /// Returns a [DecQuad] initialized from [u32] value.
  ///
  /// Result is exact, without any errors or exceptions.
  ///
  fn from(n: u32) -> Self {
    let mut dq = DEC_QUAD_POSITIVE_ZERO;
    let mut unsigned = n;
    let mut encoded = BIN2DPD[(unsigned % 1000) as usize] as u64;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 10;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 20;
    unsigned /= 1000;
    encoded |= (unsigned as u64) << 30;
    unsafe {
      dq.longs[0] = encoded;
    }
    dq
  }
}

impl From<i64> for DecQuad {
  /// Returns a [DecQuad] initialized from [i64] value.
  ///
  /// Result is exact, without any errors or exceptions.
  ///
  fn from(n: i64) -> Self {
    let mut unsigned = n as u64;
    let mut dq = if n < 0 {
      unsigned = (!unsigned) + 1;
      DEC_QUAD_NEGATIVE_ZERO
    } else {
      DEC_QUAD_POSITIVE_ZERO
    };
    let mut encoded = BIN2DPD[(unsigned % 1000) as usize] as u64;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 10;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 20;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 30;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 40;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 50;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 60;
    unsafe {
      dq.longs[0] = encoded;
    }
    dq
  }
}

impl From<u64> for DecQuad {
  /// Returns a [DecQuad] initialized from [u64] value.
  ///
  /// Result is exact, without any errors or exceptions.
  ///
  fn from(n: u64) -> Self {
    let mut dq = DEC_QUAD_POSITIVE_ZERO;
    let mut unsigned = n;
    let mut encoded = BIN2DPD[(unsigned % 1000) as usize] as u64;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 10;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 20;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 30;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 40;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 50;
    unsigned /= 1000;
    encoded |= (BIN2DPD[(unsigned % 1000) as usize] as u64) << 60;
    unsafe {
      dq.longs[0] = encoded;
      dq.longs[1] |= (unsigned >> 4) as u64;
    }
    dq
  }
}

#[inline(always)]
pub fn get_econ(dec_quad: &DecQuad) -> i32 {
  let src_hi = unsafe { dec_quad.words[3] };
  ((src_hi & 0x03FFFFFF) >> (32 - 6 - DEC_QUAD_ECONL)) as i32
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem::size_of;

  macro_rules! assert_from {
    ($expected:expr, $actual:expr) => {
      assert_eq!($expected, format!("{:?}", DecQuad::from($actual)))
    };
  }

  #[test]
  fn test_dec_quad_default() {
    let dec_quad = DecQuad::default();
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 34], unsafe { dec_quad.bytes });
  }

  #[test]
  fn test_dec_quad_union_size() {
    assert_eq!(128, 8 * size_of::<DecQuad>());
  }

  #[test]
  fn test_dec_quad_from_i8() {
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 A8]", i8::MIN);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", -100_i16);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", -10_i16);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", -1_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", -0_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 A7]", i8::MAX);
  }

  #[test]
  fn test_dec_quad_from_u8() {
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 01 55]", u8::MAX);
  }

  #[test]
  fn test_dec_quad_from_i16() {
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 CB E8]", i16::MIN);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 40 00]", -10_000_i16);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", -1_000_i16);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", -100_i16);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", -10_i16);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", -1_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", -0_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", 1_000_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 40 00]", 10_000_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 CB E7]", i16::MAX);
  }

  #[test]
  fn test_dec_quad_from_u16() {
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", 1_000_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 40 00]", 10_000_i16);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 01 96 B5]", u16::MAX);
  }

  #[test]
  fn test_dec_quad_from_i32() {
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 8C 78 AF 48]", i32::MIN);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", -1_000_000_i32);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", -1_000_i32);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", -100_i32);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", -10_i32);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", -1_i32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", -0_i32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_i32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_i32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_i32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_i32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", 1_000_i32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", 1_000_000_i32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 8C 78 AF 47]", i32::MAX);
  }

  #[test]
  fn test_dec_quad_from_u32() {
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_u32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_u32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_u32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_u32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", 1_000_u32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", 1_000_000_u32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 8C 78 AF 47]", i32::MAX as u32);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 01 15 AF B5 5B]", u32::MAX);
  }

  #[test]
  fn test_dec_quad_from_i64() {
    assert_from!("[A2 08 00 00 00 00 00 00 94 8D F2 0D A5 CF D4 2E]", i64::MIN);
    assert_from!("[A2 08 00 00 00 00 00 00 10 00 00 00 00 00 00 00]", -1_000_000_000_000_000_000_i64);
    assert_from!("[A2 08 00 00 00 00 00 00 00 04 00 00 00 00 00 00]", -1_000_000_000_000_000_i64);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 01 00 00 00 00 00]", -1_000_000_000_000_i64);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]", -1_000_000_000_i64);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", -1_000_000_i64);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", -1_000_i64);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", -100_i64);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", -10_i64);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", -1_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", -0_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", 1_000_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", 1_000_000_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]", 1_000_000_000_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 01 00 00 00 00 00]", 1_000_000_000_000_i64);
    assert_from!("[22 08 00 00 00 00 00 00 00 04 00 00 00 00 00 00]", 1_000_000_000_000_000_i64);
    assert_from!("[22 08 00 00 00 00 00 00 10 00 00 00 00 00 00 00]", 1_000_000_000_000_000_000_i64);
    assert_from!("[22 08 00 00 00 00 00 00 94 8D F2 0D A5 CF D7 0D]", i64::MAX);
  }

  #[test]
  fn test_dec_quad_from_u64() {
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_u64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_u64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_u64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_u64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]", 1_000_u64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]", 1_000_000_u64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]", 1_000_000_000_u64);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 01 00 00 00 00 00]", 1_000_000_000_000_u64);
    assert_from!("[22 08 00 00 00 00 00 00 00 04 00 00 00 00 00 00]", 1_000_000_000_000_000_u64);
    assert_from!("[22 08 00 00 00 00 00 00 10 00 00 00 00 00 00 00]", 1_000_000_000_000_000_000_u64);
    assert_from!("[22 08 00 00 00 00 00 00 19 33 74 81 DF 0B 74 69]", 1_844_674_407_370_955_069_u64);
    assert_from!("[22 08 00 00 00 00 00 00 94 8D F2 0D A5 CF D7 0D]", 9_223_372_036_854_775_807_u64);
    assert_from!("[22 08 00 00 00 00 00 01 89 1B C4 1C F8 9B 43 10]", 18_446_744_073_709_550_610_u64);
    assert_from!("[22 08 00 00 00 00 00 00 63 22 9C C6 D3 6A 5D 05]", u64::MAX / 3);
    assert_from!("[22 08 00 00 00 00 00 01 89 1B C4 1C F8 9B 47 15]", u64::MAX);
  }
}
