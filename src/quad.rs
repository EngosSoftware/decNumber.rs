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
#[derive(Copy, Clone)]
pub union DecQuad {
  pub bytes: [u8; 16],
  pub shorts: [u16; 8],
  pub words: [u32; 4],
  pub longs: [u64; 2],
}

impl std::fmt::Debug for DecQuad {
  /// Converts [DecQuad] to a string containing hexadecimal bytes.
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

impl DecQuad {
  ///
  pub fn is_negative(&self) -> bool {
    unsafe { (self.bytes[15] & 0x80) > 0 }
  }
}

impl Default for DecQuad {
  /// Initializes [DecQuad] with zeros.
  fn default() -> Self {
    DEC_QUAD_POSITIVE_ZERO
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

  #[test]
  fn test_dec_quad_default() {
    let dec_quad = DecQuad::default();
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 34], unsafe { dec_quad.bytes });
  }

  #[test]
  fn test_dec_quad_union_size() {
    assert_eq!(128, 8 * size_of::<DecQuad>());
  }
}
