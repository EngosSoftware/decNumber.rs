#![feature(test)]

extern crate test;

use dec_number::dec_quad::{dec_quad_from_i32, DecQuad};
use test::Bencher;

#[bench]
fn bench_dec_quad_from_i32(b: &mut Bencher) {
  let mut q = DecQuad::default();
  b.iter(|| {
    dec_quad_from_i32(&mut q, 255);
  });
}
