#![feature(test)]

extern crate test;

use dec_number::Decimal128;
use test::Bencher;

#[bench]
fn bench_decimal128_from_string_empty(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("");
  });
}

#[bench]
fn bench_decimal128_from_string_inf(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("inf");
  });
}

#[bench]
fn bench_decimal128_from_string_infinity(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("infinity");
  });
}

#[bench]
fn bench_decimal128_from_string_minus_inf(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("-inf");
  });
}

#[bench]
fn bench_decimal128_from_string_minus_infinity(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("-infinity");
  });
}

#[bench]
fn bench_decimal128_from_string_plus_inf(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("+inf");
  });
}

#[bench]
fn bench_decimal128_from_string_plus_infinity(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("+infinity");
  });
}

#[bench]
fn bench_decimal128_from_string_nan(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("nan");
  });
}

#[bench]
fn bench_decimal128_from_string_minus_nan(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("-nan");
  });
}

#[bench]
fn bench_decimal128_from_string_plus_nan(b: &mut Bencher) {
  b.iter(|| {
    let _ = Decimal128::from("+nan");
  });
}
