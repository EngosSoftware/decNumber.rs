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

use crate::dec_quad::DecQuad;
use std::fmt::Debug;

#[repr(C)]
pub struct Decimal128(DecQuad);

impl Decimal128 {
  /// Returns new [Decimal128] set to zero.
  pub fn zero() -> Self {
    Self(DecQuad::zero())
  }
  /// Returns new [Decimal128] set to negative zero.
  pub fn negative_zero() -> Self {
    Self(DecQuad::negative_zero())
  }
  /// Returns the absolute value of this [Decimal128].
  pub fn abs(&self) -> Self {
    Self(self.0.abs())
  }
  /// Returns `true` if this [Decimal128] is less than zero and not a `NaN`, or `false` otherwise.
  pub fn is_negative(&self) -> bool {
    self.0.is_negative()
  }
  /// Returns `true` if this [Decimal128] has a sign, or `false` otherwise.
  /// Note that zeros and NaNs may also have a sign.
  pub fn is_signed(&self) -> bool {
    self.0.is_signed()
  }
  /// Returns `true` if this [Decimal128] is a zero, or `false` otherwise.
  pub fn is_zero(&self) -> bool {
    self.0.is_zero()
  }
  /// Returns `true` if this [Decimal128] is a NaN (quiet or signaling), or `false` otherwise.
  pub fn is_nan(&self) -> bool {
    self.0.is_nan()
  }
}

impl Default for Decimal128 {
  /// The default value for [Decimal128] is positive zero.
  fn default() -> Self {
    Self(DecQuad::default())
  }
}

impl Debug for Decimal128 {
  /// Converts [Decimal128] to a string in the form of hexadecimal bytes separated with spaces.
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
