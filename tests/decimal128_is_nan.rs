use dec_number::Decimal128;

#[test]
fn decimal_128_is_nan_default() {
  assert!(!Decimal128::default().is_nan());
}
