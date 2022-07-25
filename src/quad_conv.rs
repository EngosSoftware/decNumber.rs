use crate::dec_quad::{
  DecQuad, DEC_QUAD_NEGATIVE_INFINITY, DEC_QUAD_NEGATIVE_NAN,
  DEC_QUAD_NEGATIVE_ZERO, DEC_QUAD_POSITIVE_INFINITY, DEC_QUAD_POSITIVE_NAN,
  DEC_QUAD_POSITIVE_ZERO,
};
use crate::dpd::{BIN2DPD, DPD2BIN};

impl From<i8> for DecQuad {
  /// Returns a [DecQuad] initialized from [i8] value.
  /// Result is exact, without any errors or exceptions.
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
  /// Result is exact, without any errors or exceptions.
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
  /// Result is exact, without any errors or exceptions.
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
  /// Result is exact, without any errors or exceptions.
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
  /// Result is exact, without any errors or exceptions.
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
  /// Result is exact, without any errors or exceptions.
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
  /// Result is exact, without any errors or exceptions.
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
  /// Result is exact, without any errors or exceptions.
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
      dq.bytes[8] = (unsigned >> 4) as u8;
    }
    dq
  }
}

//const NUMBER_PATTERN: &str = r#"(?P<sign>(-|+))?(?P<year>[1-9][0-9]{3,8})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})"#;
// const NUMBER_PATTERN: &str =
//   r#"(?P<sign>(-|+))?(?P<digits>[0-9])-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})"#;

impl From<&str> for DecQuad {
  fn from(s: &str) -> Self {
    // when the string is empty, just return NaN
    if s.is_empty() {
      return DEC_QUAD_POSITIVE_NAN;
    }
    match s.to_uppercase().trim() {
      "INF" | "+INF" | "INFINITY" | "+INFINITY" => DEC_QUAD_POSITIVE_INFINITY,
      "-INF" | "-INFINITY" => DEC_QUAD_NEGATIVE_INFINITY,
      "NAN" | "+NAN" => DEC_QUAD_POSITIVE_NAN,
      "-NAN" => DEC_QUAD_NEGATIVE_NAN,
      _ => DEC_QUAD_POSITIVE_ZERO,
    }
  }
}

impl TryFrom<DecQuad> for i8 {
  type Error = ();
  ///
  fn try_from(dq: DecQuad) -> Result<Self, Self::Error> {
    let mut unsigned = unsafe { DPD2BIN[dq.shorts[0] as usize] };
    if dq.is_negative() {
      unsigned = (!unsigned) + 1;
    }
    if (unsigned as i16) >= (i8::MIN as i16)
      && (unsigned as i16) <= (i8::MAX as i16)
    {
      Ok(unsigned as i8)
    } else {
      Err(())
    }
  }
}

impl TryFrom<DecQuad> for i64 {
  type Error = ();
  ///
  fn try_from(dq: DecQuad) -> Result<Self, Self::Error> {
    let encoded = unsafe { dq.longs[0] };
    let mut unsigned = DPD2BIN[((encoded >> 60) & 0x2FF) as usize] as u64;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 50) & 0x3FF) as usize] as u64;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 40) & 0x3FF) as usize] as u64;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 30) & 0x3FF) as usize] as u64;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 20) & 0x3FF) as usize] as u64;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 10) & 0x3FF) as usize] as u64;
    unsigned *= 1000;
    unsigned += DPD2BIN[(encoded & 0x3FF) as usize] as u64;
    if dq.is_negative() {
      unsigned = (!unsigned) + 1;
    }
    Ok(unsigned as i64)
  }
}

impl TryFrom<DecQuad> for u64 {
  type Error = ();
  ///
  fn try_from(dq: DecQuad) -> Result<Self, Self::Error> {
    let (encoded, mut unsigned) =
      unsafe { (dq.longs[0], (dq.bytes[8] * 10) as u128) };
    unsigned += DPD2BIN[((encoded >> 60) & 0x2FF) as usize] as u128;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 50) & 0x3FF) as usize] as u128;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 40) & 0x3FF) as usize] as u128;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 30) & 0x3FF) as usize] as u128;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 20) & 0x3FF) as usize] as u128;
    unsigned *= 1000;
    unsigned += DPD2BIN[((encoded >> 10) & 0x3FF) as usize] as u128;
    unsigned *= 1000;
    unsigned += DPD2BIN[(encoded & 0x3FF) as usize] as u128;
    Ok(unsigned as u64)
  }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
  use super::*;

  macro_rules! assert_from {
    ($expected:expr, $actual:expr) => {
      assert_eq!($expected, format!("{:?}", DecQuad::from($actual)))
    };
  }

  #[test]
  fn test_dec_quad_from_i8() {
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 A8]", i8::MIN);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", -100_i8);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", -10_i8);
    assert_from!("[A2 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", -1_i8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", -0_i8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_i8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_i8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_i8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_i8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 A7]", i8::MAX);
  }

  #[test]
  fn test_i8_from_dec_quad() {
    assert_eq!(Err(()), i8::try_from(DecQuad::from(i8::MIN as i16 - 1)));
    assert_eq!(Ok(i8::MIN), i8::try_from(DecQuad::from(i8::MIN)));
    assert_eq!(Ok(-100_i8), i8::try_from(DecQuad::from(-100_i8)));
    assert_eq!(Ok(-10_i8), i8::try_from(DecQuad::from(-10_i8)));
    assert_eq!(Ok(-1_i8), i8::try_from(DecQuad::from(-1_i8)));
    assert_eq!(Ok(-0_i8), i8::try_from(DecQuad::from(-0_i8)));
    assert_eq!(Ok(0_i8), i8::try_from(DecQuad::from(0_i8)));
    assert_eq!(Ok(1_i8), i8::try_from(DecQuad::from(1_i8)));
    assert_eq!(Ok(10_i8), i8::try_from(DecQuad::from(10_i8)));
    assert_eq!(Ok(100_i8), i8::try_from(DecQuad::from(100_i8)));
    assert_eq!(Ok(i8::MAX), i8::try_from(DecQuad::from(i8::MAX)));
    assert_eq!(Err(()), i8::try_from(DecQuad::from(i8::MAX as i16 + 1)));
  }

  #[test]
  fn test_dec_quad_from_u8() {
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", 0_u8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]", 1_u8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]", 10_u8);
    assert_from!("[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]", 100_u8);
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
  fn test_i64_from_dec_quad() {
    assert_eq!(Ok(i64::MIN), i64::try_from(DecQuad::from(i64::MIN)));
    assert_eq!(Ok(-1_000_000_000_000_000_000_i64), i64::try_from(DecQuad::from(-1_000_000_000_000_000_000_i64)));
    assert_eq!(Ok(-1_000_000_000_000_000_i64), i64::try_from(DecQuad::from(-1_000_000_000_000_000_i64)));
    assert_eq!(Ok(-1_000_000_000_000_i64), i64::try_from(DecQuad::from(-1_000_000_000_000_i64)));
    assert_eq!(Ok(-1_000_000_000_i64), i64::try_from(DecQuad::from(-1_000_000_000_i64)));
    assert_eq!(Ok(-1_000_000_i64), i64::try_from(DecQuad::from(-1_000_000_i64)));
    assert_eq!(Ok(-1_000_i64), i64::try_from(DecQuad::from(-1_000_i64)));
    assert_eq!(Ok(-100_i64), i64::try_from(DecQuad::from(-100_i64)));
    assert_eq!(Ok(-10_i64), i64::try_from(DecQuad::from(-10_i64)));
    assert_eq!(Ok(-1_i64), i64::try_from(DecQuad::from(-1_i64)));
    assert_eq!(Ok(-0_i64), i64::try_from(DecQuad::from(-0_i64)));
    assert_eq!(Ok(0_i64), i64::try_from(DecQuad::from(0_i64)));
    assert_eq!(Ok(1_i64), i64::try_from(DecQuad::from(1_i64)));
    assert_eq!(Ok(10_i64), i64::try_from(DecQuad::from(10_i64)));
    assert_eq!(Ok(100_i64), i64::try_from(DecQuad::from(100_i64)));
    assert_eq!(Ok(1_000_i64), i64::try_from(DecQuad::from(1_000_i64)));
    assert_eq!(Ok(1_000_000_i64), i64::try_from(DecQuad::from(1_000_000_i64)));
    assert_eq!(Ok(1_000_000_000_i64), i64::try_from(DecQuad::from(1_000_000_000_i64)));
    assert_eq!(Ok(1_000_000_000_000_i64), i64::try_from(DecQuad::from(1_000_000_000_000_i64)));
    assert_eq!(Ok(1_000_000_000_000_000_i64), i64::try_from(DecQuad::from(1_000_000_000_000_000_i64)));
    assert_eq!(Ok(1_000_000_000_000_000_000_i64), i64::try_from(DecQuad::from(1_000_000_000_000_000_000_i64)));
    assert_eq!(Ok(i64::MAX), i64::try_from(DecQuad::from(i64::MAX)));
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

  #[test]
  fn test_u64_from_dec_quad() {
    assert_eq!(Ok(u64::MIN), u64::try_from(DecQuad::from(u64::MIN)));
    assert_eq!(Ok(0_u64), u64::try_from(DecQuad::from(0_u64)));
    assert_eq!(Ok(1_u64), u64::try_from(DecQuad::from(1_u64)));
    assert_eq!(Ok(1_000_000_000_000_000_000_u64), u64::try_from(DecQuad::from(1_000_000_000_000_000_000_u64)));
    assert_eq!(Ok(1_844_674_407_370_955_069_u64), u64::try_from(DecQuad::from(1_844_674_407_370_955_069_u64)));
    assert_eq!(Ok(9_223_372_036_854_775_807_u64), u64::try_from(DecQuad::from(9_223_372_036_854_775_807_u64)));
    assert_eq!(Ok(18_446_744_073_709_550_610_u64), u64::try_from(DecQuad::from(18_446_744_073_709_550_610_u64)));
    assert_eq!(Ok(u64::MAX), u64::try_from(DecQuad::from(u64::MAX)));
  }

  #[test]
  fn test_u64_from_string() {
    assert_from!("[7C 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from(""));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("inf"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("INF"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("InF"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("+inf"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("+INF"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("+InF"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("infinity"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("INFINITY"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("iNfInItY"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("+infinity"));
    assert_from!("[78 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("+InFiNiTy"));
    assert_from!("[7C 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("nan"));
    assert_from!("[7C 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("NAN"));
    assert_from!("[7C 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("NaN"));
    assert_from!("[7C 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("+nan"));
    assert_from!("[7C 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("+NAN"));
    assert_from!("[7C 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("+nAn"));
    assert_from!("[FC 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("-nan"));
    assert_from!("[FC 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("-NAN"));
    assert_from!("[FC 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00]", DecQuad::from("-NaN"));
  }
}
