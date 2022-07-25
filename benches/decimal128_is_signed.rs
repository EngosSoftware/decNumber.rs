#![feature(test)]

extern crate test;

use dec_number::Decimal128;
use test::Bencher;

#[bench]
fn bench_decimal128_abs_i8_min(b: &mut Bencher) {
  let d = Decimal128::from(i8::MIN);
  b.iter(|| {
    let _ = d.is_signed();
  });
}

#[bench]
fn bench_decimal128_abs_i8_max(b: &mut Bencher) {
  let d = Decimal128::from(i8::MAX);
  b.iter(|| {
    let _ = d.is_signed();
  });
}
