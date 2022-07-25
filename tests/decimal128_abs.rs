use dec_number::Decimal128;

#[test]
fn decimal_128_abs() {
  let actual: Decimal128 = (-123_i8).into();
  assert!(actual.is_negative());
  let expected: Decimal128 = 1_i8.into();
  assert!(!expected.is_negative());
  let a = actual.abs();
  assert!(!a.is_negative());
}
