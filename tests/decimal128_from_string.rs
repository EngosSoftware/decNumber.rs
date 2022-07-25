use dec_number::Decimal128;

#[test]
fn decimal_128_from_string() {
  assert!(Decimal128::from("").is_nan());
}
