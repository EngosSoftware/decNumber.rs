use dec_number::Decimal128;

#[test]
fn decimal_128_is_negative_default() {
  assert!(!Decimal128::default().is_negative());
}

#[test]
fn decimal_128_is_negative_i8() {
  assert!(Decimal128::from(i8::MIN).is_negative());
  assert!(Decimal128::from(-1_i8).is_negative());
  assert!(!Decimal128::from(0_i8).is_negative());
  assert!(!Decimal128::from(1_i8).is_negative());
  assert!(!Decimal128::from(i8::MAX).is_negative());
}

#[test]
fn decimal_128_is_negative_u8() {
  assert!(!Decimal128::from(u8::MIN).is_negative());
  assert!(!Decimal128::from(0_u8).is_negative());
  assert!(!Decimal128::from(1_u8).is_negative());
  assert!(!Decimal128::from(u8::MAX).is_negative());
}

#[test]
fn decimal_128_is_negative_u16() {
  assert!(!Decimal128::from(u16::MIN).is_negative());
  assert!(!Decimal128::from(0_u16).is_negative());
  assert!(!Decimal128::from(1_u16).is_negative());
  assert!(!Decimal128::from(u16::MAX).is_negative());
}
