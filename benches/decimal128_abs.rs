#![feature(test)]

extern crate test;

use dec_number::Decimal128;
use test::Bencher;

#[bench]
fn bench_decimal128_abs_i8(b: &mut Bencher) {
  let d = Decimal128::from(i8::MIN);
  b.iter(|| {
    let _ = d.abs();
  });
}

#[bench]
fn bench_decimal128_abs_i16(b: &mut Bencher) {
  let d = Decimal128::from(i16::MIN);
  b.iter(|| {
    let _ = d.abs();
  });
}
