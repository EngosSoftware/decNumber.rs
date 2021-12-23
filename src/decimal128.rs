/*
 * MIT License
 *
 * Copyright (c) 2021 Dariusz Depta Engos Software
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! `Decimal128`

use crate::quad::DecQuad;

#[repr(C)]
pub struct Decimal128(DecQuad);

impl std::fmt::Debug for Decimal128 {
  /// Converts [Decimal128] to a string of hexadecimal bytes.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.0)
  }
}

impl From<i8> for Decimal128 {
  /// Returns a [Decimal128] initialized from [i8] value.
  fn from(n: i8) -> Self {
    Self(DecQuad::from(n))
  }
}

impl From<u8> for Decimal128 {
  /// Returns a [Decimal128] initialized from [u8] value.
  fn from(n: u8) -> Self {
    Self(DecQuad::from(n))
  }
}

impl From<i16> for Decimal128 {
  /// Returns a [Decimal128] initialized from [i16] value.
  fn from(n: i16) -> Self {
    Self(DecQuad::from(n))
  }
}

impl From<u16> for Decimal128 {
  /// Returns a [Decimal128] initialized from [u16] value.
  fn from(n: u16) -> Self {
    Self(DecQuad::from(n))
  }
}

impl From<i32> for Decimal128 {
  /// Returns a [Decimal128] initialized from [i32] value.
  fn from(n: i32) -> Self {
    Self(DecQuad::from(n))
  }
}

impl From<u32> for Decimal128 {
  /// Returns a [Decimal128] initialized from [u32] value.
  fn from(n: u32) -> Self {
    Self(DecQuad::from(n))
  }
}

impl From<i64> for Decimal128 {
  /// Returns a [Decimal128] initialized from [i64] value.
  fn from(n: i64) -> Self {
    Self(DecQuad::from(n))
  }
}

impl From<u64> for Decimal128 {
  /// Returns a [Decimal128] initialized from [u64] value.
  fn from(n: u64) -> Self {
    Self(DecQuad::from(n))
  }
}
