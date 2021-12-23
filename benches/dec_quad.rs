#![feature(test)]

extern crate test;

use dec_number::quad::DecQuad;
use test::Bencher;

#[bench]
fn bench_dec_quad_from_i8(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::from(1_i8);
  });
}

#[bench]
fn bench_dec_quad_from_u8(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::from(1_u8);
  });
}

#[bench]
fn bench_dec_quad_from_i16(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::from(1_i16);
  });
}

#[bench]
fn bench_dec_quad_from_u16(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::from(1_u16);
  });
}

#[bench]
fn bench_dec_quad_from_i32(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::from(1_i32);
  });
}

#[bench]
fn bench_dec_quad_from_u32(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::from(1_u32);
  });
}

#[bench]
fn bench_dec_quad_from_i64(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::from(1_i64);
  });
}

#[bench]
fn bench_dec_quad_from_u64(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::from(1_u64);
  });
}

#[bench]
fn bench_i8_from_dec_quad(b: &mut Bencher) {
  let dq = DecQuad::from(1_i8);
  b.iter(move || {
    let _ = i8::try_from(dq);
  });
}

#[bench]
fn bench_i64_from_dec_quad(b: &mut Bencher) {
  let dq = DecQuad::from(1_i64);
  b.iter(move || {
    let _ = i64::try_from(dq);
  });
}

#[bench]
fn bench_u64_from_dec_quad(b: &mut Bencher) {
  let dq = DecQuad::from(1_u64);
  b.iter(move || {
    let _ = u64::try_from(dq);
  });
}
