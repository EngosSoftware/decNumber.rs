use dec_number::Decimal128;

#[test]
fn decimal_128_is_zero_default() {
  assert!(Decimal128::default().is_zero());
  assert!(Decimal128::zero().is_zero());
  assert!(Decimal128::negative_zero().is_zero());
}

#[test]
fn decimal_128_is_zero_i8() {
  assert!(!Decimal128::from(i8::MIN).is_zero());
  assert!(Decimal128::from(0_i8).is_zero());
  assert!(Decimal128::from(-0_i8).is_zero());
  assert!(!Decimal128::from(i8::MAX).is_zero());
}
