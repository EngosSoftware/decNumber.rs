use dec_number::Decimal128;

#[test]
fn decimal_128_is_signed_default() {
  assert!(!Decimal128::default().is_signed());
}

#[test]
fn decimal_128_is_signed_i8() {
  assert!(Decimal128::from(i8::MIN).is_signed());
  assert!(Decimal128::from(-1_i8).is_signed());
  assert!(!Decimal128::from(0_i8).is_signed());
  assert!(!Decimal128::from(1_i8).is_signed());
  assert!(!Decimal128::from(i8::MAX).is_signed());
}
