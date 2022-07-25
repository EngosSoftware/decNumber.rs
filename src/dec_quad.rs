use crate::dec_common::{canonical_dpd_off, canonical_dpd_two, Uint};
use std::fmt::Debug;

// ----------------------------------------------------------------
// Parameters for DecQuads
// ----------------------------------------------------------------
/// Length in bytes of the DecQuad union.
pub const DEC_QUAD_BYTES: usize = 16;
/// Length in words of the DecQuad union.
pub const DEC_QUAD_WORDS: usize = DEC_QUAD_BYTES / 4;
/// Maximum precision (digits).
pub const DEC_QUAD_PMAX: i32 = 34;
/// Minimum adjusted exponent.
pub const DEC_QUAD_EMIN: i32 = -6143;
/// Maximum adjusted exponent.
pub const DEC_QUAD_EMAX: i32 = 6144;
/// Maximum exponent digits.
pub const DEC_QUAD_EMAX_D: i32 = 4;
/// Bias for the exponent.
pub const DEC_QUAD_BIAS: i32 = 6176;
/// Maximum string length, +1.
pub const DEC_QUAD_STRING: i32 = 43;
/// Exponent continuation length.
pub const DEC_QUAD_ECONL: i32 = 12;
/// Count of declets.
pub const DEC_QUAD_DECLETS: i32 = 11;
/// Highest biased exponent (Elimit - 1).
pub const DEC_QUAD_EHIGH: i32 =
  DEC_QUAD_EMAX + DEC_QUAD_BIAS - (DEC_QUAD_PMAX - 1);

// ----------------------------------------------------------------
// Shared constants
// ----------------------------------------------------------------

// Sign and special values.
// Top 32-bits are used.
// Last two bits are don't-care for Infinity on input.
// Last bit don't-care for NaNs.

/// 1 00000 00 Sign.
pub const DECFLOAT_SIGN: Uint = 0x80000000;
/// 0 11111 00 NaN generic.
pub const DECFLOAT_NAN: Uint = 0x7c000000;
/// 0 11111 00 qNaN.
pub const DECFLOAT_QNAN: Uint = 0x7c000000;
/// 0 11111 10 sNaN.
pub const DECFLOAT_SNAN: Uint = 0x7e000000;
/// 0 11110 00 Infinity.
pub const DECFLOAT_INF: Uint = 0x78000000;
/// Minimum special value, specials are all >= [DECFLOAT_MIN_SP].
pub const DECFLOAT_MIN_SP: Uint = 0x78000000;

///
#[rustfmt::skip]
pub(crate) const DEC_QUAD_POSITIVE_ZERO: DecQuad = DecQuad {
  bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22],
};

///
#[rustfmt::skip]
pub(crate) const DEC_QUAD_NEGATIVE_ZERO: DecQuad = DecQuad {
  bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0xA2 ],
};

/// The decimal 128-bit type accessible by all sizes.
#[repr(C)]
#[derive(Copy, Clone)]
pub union DecQuad {
  pub bytes: [u8; DEC_QUAD_BYTES],
  pub shorts: [u16; DEC_QUAD_BYTES / 2],
  pub words: [Uint; DEC_QUAD_BYTES / 4],
  pub longs: [u64; DEC_QUAD_BYTES / 8],
}

impl DecQuad {
  /// Returns new [DecQuad] set to zero.
  pub fn zero() -> Self {
    DEC_QUAD_POSITIVE_ZERO
  }
  /// Returns new [DecQuad] set to negative zero.
  pub fn negative_zero() -> Self {
    DEC_QUAD_NEGATIVE_ZERO
  }
  /// Returns the absolute value of this [DecQuad].
  pub fn abs(&self) -> DecQuad {
    let mut canonical = self.canonical();
    unsafe {
      #[cfg(target_endian = "little")]
      {
        canonical.bytes[DEC_QUAD_BYTES - 1] &= !0x80;
      }
      #[cfg(target_endian = "big")]
      {
        canonical.bytes[0] &= !0x80;
      }
    };
    canonical
  }
  /// Returns `true` if this [DecQuad] is less than zero and not a `NaN`,
  /// or `false` otherwise.
  pub fn is_negative(&self) -> bool {
    unsafe { (self.bytes[15] & 0x80) > 0 }
  }
  /// Returns `true` if this [DecQuad] has a sign, or 0 otherwise.
  /// Note that zeros and NaNs may also have a sign.
  pub fn is_signed(&self) -> bool {
    unsafe {
      #[cfg(target_endian = "little")]
      {
        self.words[DEC_QUAD_WORDS - 1] & DECFLOAT_SIGN != 0
      }
      #[cfg(target_endian = "big")]
      {
        self.words[0] & DECFLOAT_SIGN != 0
      }
    }
  }
  /// Returns `true` if this [DecQuad] is a zero, or 0 otherwise.
  pub fn is_zero(&self) -> bool {
    self.get_word(3) == &0
      && self.get_word(2) == &0
      && self.get_word(1) == &0
      && (self.get_word(0) & 0x1c003fff) == 0
      && (self.get_word(0) & 0x60000000) != 0x60000000
  }
  /// Returns a canonical copy of this [DecQuad].
  fn canonical(&self) -> DecQuad {
    let source_hi = self.get_word(0);
    let source_mh = self.get_word(1);
    let source_ml = self.get_word(2);
    let source_lo = self.get_word(3);
    if canonical_dpd_off(source_hi, 4)
      & canonical_dpd_two(source_hi, source_mh, 26)
      & canonical_dpd_off(source_mh, 16)
      & canonical_dpd_off(source_mh, 6)
      & canonical_dpd_two(source_mh, source_ml, 28)
      & canonical_dpd_off(source_ml, 18)
      & canonical_dpd_off(source_ml, 8)
      & canonical_dpd_two(source_ml, source_lo, 30)
      & canonical_dpd_off(source_lo, 20)
      & canonical_dpd_off(source_lo, 10)
      & canonical_dpd_off(source_lo, 0)
    {
      *self
    } else {
      unimplemented!("non canonical form in canonical() function")
    }
  }
  ///
  pub fn get_econ(&self) -> i32 {
    let src_hi = unsafe { self.words[3] };
    ((src_hi & 0x03FFFFFF) >> (32 - 6 - DEC_QUAD_ECONL)) as i32
  }
  /// Returns a reference to a word with specified offset.
  fn get_word(&self, offset: usize) -> &Uint {
    unsafe {
      #[cfg(target_endian = "little")]
      {
        &self.words[DEC_QUAD_WORDS - 1 - offset]
      }
      #[cfg(target_endian = "big")]
      {
        &self.words[offset]
      }
    }
  }
}

impl Default for DecQuad {
  /// The default value for [DecQuad] is positive zero.
  fn default() -> Self {
    DEC_QUAD_POSITIVE_ZERO
  }
}

impl Debug for DecQuad {
  /// Converts [DecQuad] to a string in the form of hexadecimal bytes separated with spaces.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem::size_of;

  /// Default value for [DecQuad] should be positive zero.
  #[test]
  fn test_dec_quad_default() {
    let dec_quad = DecQuad::default();
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 34], unsafe {
      dec_quad.bytes
    });
  }

  /// Union size for [DecQuad] must be 128 bits.
  #[test]
  fn test_dec_quad_union_size() {
    assert_eq!(128, 8 * size_of::<DecQuad>());
    assert_eq!(128, 8 * size_of::<[u8; DEC_QUAD_BYTES]>());
    assert_eq!(128, 8 * size_of::<[u16; DEC_QUAD_BYTES / 2]>());
    assert_eq!(128, 8 * size_of::<[u32; DEC_QUAD_BYTES / 4]>());
    assert_eq!(128, 8 * size_of::<[u64; DEC_QUAD_BYTES / 8]>());
  }
}
