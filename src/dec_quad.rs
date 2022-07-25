use crate::dec_common::{
  canonical_dpd_off, canonical_dpd_two, Ubyte, Uint, QUADZERO,
};
use crate::dpd::{BIN2DPD, DPD2BIN};
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
/* Compile-time computes of the exponent continuation field masks */
/* full exponent continuation field:                              */
pub const ECON_MASK: Uint =
  (0x03ffffff >> (32 - 6 - DEC_QUAD_ECONL)) << (32 - 6 - DEC_QUAD_ECONL);
/* same, not including its first digit (the qNaN/sNaN selector):  */
pub const ECON_NAN_MASK: Uint =
  (0x01ffffff >> (32 - 6 - DEC_QUAD_ECONL)) << (32 - 6 - DEC_QUAD_ECONL);
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
    let mut canonical = self.as_canonical();
    canonical.set_byte(0, canonical.get_byte(0) & !0x80);
    canonical
  }

  /// Returns `true` if this [DecQuad] is less than zero and not a `NaN`, or `false` otherwise.
  pub fn is_negative(&self) -> bool {
    self.is_signed() && !self.is_zero() && !self.is_nan()
  }

  /// Returns `true` if this [DecQuad] has a sign, or `false` otherwise.
  /// Note that zeros and NaNs may also have a sign.
  pub fn is_signed(&self) -> bool {
    (self.get_word(0) & DECFLOAT_SIGN) != 0
  }

  /// Returns `true` if this [DecQuad] is a zero, or `false` otherwise.
  pub fn is_zero(&self) -> bool {
    self.get_word(3) == &0
      && self.get_word(2) == &0
      && self.get_word(1) == &0
      && (self.get_word(0) & 0x1c003fff) == 0
      && (self.get_word(0) & 0x60000000) != 0x60000000
  }

  /// Returns `true` if the coefficient continuation of this [DecQuad] is zero, or `false` otherwise.
  pub fn is_cc_zero(&self) -> bool {
    self.get_word(3) == &0
      && self.get_word(2) == &0
      && self.get_word(1) == &0
      && (self.get_word(0) & 0x00003fff) == 0
  }

  /// Returns `true` if this [DecQuad] is a NaN (quiet or signaling), or `false` otherwise.
  pub fn is_nan(&self) -> bool {
    (self.get_word(0) & DECFLOAT_NAN) == DECFLOAT_NAN
  }

  /// Returns `true` if the encoding of this [DecQuad] is an infinity, or `false` otherwise.
  pub fn is_infinite(&self) -> bool {
    (self.get_word(0) & DECFLOAT_NAN) == DECFLOAT_INF
  }

  /// Returns a canonical copy of this [DecQuad].
  pub fn as_canonical(&self) -> DecQuad {
    let mut result = *self;
    if result.is_special() {
      if result.is_infinite() {
        // clean Infinity
        result.set_infinity();
        return result;
      }
      // is a NaN
      result.set_word(0, result.get_word(0) & !ECON_NAN_MASK);
      if result.is_cc_zero() {
        return result;
      }
      // drop through to check payload in all other cases
    }
    // return quickly if the coefficient continuation is canonical
    let source_hi = result.get_word(0);
    let source_mh = result.get_word(1);
    let source_ml = result.get_word(2);
    let source_lo = result.get_word(3);
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
      return result;
    }
    let mut in_word = DEC_QUAD_WORDS - 1; // current input word index
    let mut u_off = 0_u32; // a bit offset of declet
    let mut encode = *result.get_word(in_word);
    for _ in (0..DEC_QUAD_DECLETS).rev() {
      let mut dpd: Uint = encode >> u_off;
      u_off += 10;
      if u_off > 32 {
        // crossed Uint boundary
        in_word -= 1;
        encode = *result.get_word(in_word);
        u_off -= 32;
        dpd |= encode << (10 - u_off); // get pending bits
        dpd &= 0x3ff; // clear uninteresting bits
        if dpd < 0x16e {
          // must be canonical
          continue;
        }
        let canonical_declet = BIN2DPD[DPD2BIN[dpd as usize] as usize] as Uint; // determine canonical declet
        if canonical_declet == dpd {
          // have canonical declet
          continue;
        }
        // need to replace declet
        if u_off >= 10 {
          // all within current word
          encode &= !(0x3ff << (u_off - 10)); // clear the 10 bits ready for replace
          encode |= canonical_declet << (u_off - 10); // insert the canonical form
          result.set_word(in_word, encode); // .. and save
          continue;
        }
        // straddled words
        let mut precode = *result.get_word(in_word + 1); // get previous
        precode &= 0xffffffff >> (10 - u_off); // clear top bits
        result.set_word(
          in_word + 1,
          precode | (canonical_declet << (32 - (10 - u_off))),
        );
        encode &= 0xffffffff << u_off; // clear bottom bits
        encode |= canonical_declet >> (10 - u_off); // insert canonical
        result.set_word(in_word, encode); // .. and save
      }
    }
    result
  }

  ///
  pub fn set_infinity(&mut self) {
    // save source sign word
    let sign = *self.get_word(0);
    // clear everything to zero
    self.set_zero();
    // set infinity flag with preserved sign
    self.set_word(0, DECFLOAT_INF | (sign & DECFLOAT_SIGN));
  }

  /// Set this [DecFloat] to canonical (integer) zero +0 (q=0, c=+0).
  /// No error is possible, and no status can be set.
  pub fn set_zero(&mut self) {
    self.set_words([QUADZERO, 0, 0, 0]);
  }

  /// Returns `true` if this [DecQuad] is a special number, or `false` otherwise.
  pub fn is_special(&self) -> bool {
    (self.get_word(0) & DECFLOAT_MIN_SP) == DECFLOAT_MIN_SP
  }

  ///
  pub fn get_econ(&self) -> i32 {
    let src_hi = unsafe { self.words[3] };
    ((src_hi & 0x03FFFFFF) >> (32 - 6 - DEC_QUAD_ECONL)) as i32
  }

  /// Returns a reference to a byte with specified offset.
  fn get_byte(&self, offset: usize) -> &Ubyte {
    unsafe {
      #[cfg(target_endian = "little")]
      {
        &self.bytes[DEC_QUAD_BYTES - 1 - offset]
      }
      #[cfg(target_endian = "big")]
      {
        &self.bytes[offset]
      }
    }
  }

  /// Sets a new value to byte specified with offset.
  fn set_byte(&mut self, offset: usize, value: Ubyte) {
    unsafe {
      #[cfg(target_endian = "little")]
      {
        self.bytes[DEC_QUAD_BYTES - 1 - offset] = value;
      }
      #[cfg(target_endian = "big")]
      {
        self.bytes[offset] = value;
      }
    }
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

  /// Sets a new value to word specified with offset.
  fn set_word(&mut self, offset: usize, value: Uint) {
    unsafe {
      #[cfg(target_endian = "little")]
      {
        self.words[DEC_QUAD_WORDS - 1 - offset] = value;
      }
      #[cfg(target_endian = "big")]
      {
        self.words[offset] = value;
      }
    }
  }

  ///
  fn set_words(&mut self, values: [Uint; 4]) {
    unsafe {
      #[cfg(target_endian = "little")]
      {
        self.words[3] = values[0];
        self.words[2] = values[1];
        self.words[1] = values[2];
        self.words[0] = values[3];
      }
      #[cfg(target_endian = "big")]
      {
        self.words[0] = values[0];
        self.words[1] = values[1];
        self.words[2] = values[2];
        self.words[3] = values[3];
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
