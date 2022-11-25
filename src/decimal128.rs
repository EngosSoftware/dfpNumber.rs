/*
 * MIT License
 *
 * Copyright (c) 2022 senees
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

//! 128-bit decimal floating point arithmetic.

use dfp_number_sys::*;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

#[macro_export]
macro_rules! decnum {
  ($e:expr) => {{
    Decimal128::from(stringify!($e))
  }};
}

/// 128-bit decimal value.
#[derive(Copy, Clone)]
pub struct Decimal128(BID128);

impl Default for Decimal128 {
  /// The default value of [Decimal128] is `0` (zero).
  fn default() -> Self {
    Self::zero()
  }
}

impl fmt::Debug for Decimal128 {
  /// Converts [Decimal128] into string in debug mode.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut flags = FB_CLEAR;
    write!(f, "{}", bid128_to_string(self.0, &mut flags))
  }
}

impl fmt::Display for Decimal128 {
  /// Converts [Decimal128] into human readable string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut flags = FB_CLEAR;
    let s = bid128_to_string(self.0, &mut flags);
    let negative = s.starts_with('-');
    let mut split = s[1..].split('E');
    if let Some((sb, sa)) = split.next().zip(split.next()) {
      if let Ok(exponent) = sa.parse::<isize>() {
        let decimal_points = exponent.unsigned_abs();
        let (mut before, mut after) = if exponent < 0 {
          let digit_count = sb.len();
          if digit_count <= decimal_points {
            let before = "0".to_string();
            let mut after = "0".repeat(decimal_points - digit_count);
            after.push_str(sb.trim_end_matches('0'));
            (before, after)
          } else {
            let before = sb[..digit_count - decimal_points].to_string();
            let after = sb[digit_count - decimal_points..]
              .trim_end_matches('0')
              .to_string();
            (before, after)
          }
        } else {
          let mut before = sb.to_string();
          before.push_str(&"0".repeat(decimal_points));
          let after = "".to_string();
          (before, after)
        };
        if let Some(precision) = f.precision() {
          if after.len() < precision {
            after.push_str(&"0".repeat(precision - after.len()));
          } else {
            after = after[0..precision].to_string();
          }
        }
        if !after.is_empty() {
          before.push('.');
          before.push_str(&after);
        }
        return f.pad_integral(!negative, "", &before);
      }
    }
    f.pad(&s)
  }
}

impl Decimal128 {
  ///
  pub fn new(n: i64, s: i32) -> Self {
    Self(bid128_scalbn(bid128_from_int64(n), -s))
  }
  ///
  pub fn zero() -> Self {
    Self(bid128_from_uint32(0))
  }
  ///
  pub fn is_zero(&self) -> bool {
    bid128_is_zero(self.0)
  }
  ///
  pub fn one() -> Self {
    Self(bid128_from_uint32(1))
  }
  ///
  pub fn two() -> Self {
    Self(bid128_from_uint32(2))
  }
  ///
  pub fn ten() -> Self {
    Self(bid128_from_uint32(10))
  }
  ///
  pub fn one_hundred() -> Self {
    Self(bid128_from_uint32(100))
  }
  ///
  pub fn one_thousand() -> Self {
    Self(bid128_from_uint32(1000))
  }
  ///
  pub fn ln(&self) -> Decimal128 {
    let mut flags = FB_CLEAR;
    Self(bid128_log(self.0, RM_NEAREST_EVEN, &mut flags))
  }
  ///
  pub fn exp(&self) -> Decimal128 {
    let mut flags = FB_CLEAR;
    Self(bid128_exp(self.0, RM_NEAREST_EVEN, &mut flags))
  }
  ///
  pub fn round_dp(&self, dp: i32) -> Self {
    let q = bid128_scalbn(Self::one().0, -dp);
    let mut flags = FB_CLEAR;
    Self(bid128_quantize(self.0, q, RM_NEAREST_EVEN, &mut flags))
  }
}

impl std::ops::Neg for Decimal128 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self(bid128_negate(self.0))
  }
}

impl std::ops::Add<Self> for Decimal128 {
  type Output = Self;
  ///
  fn add(self, rhs: Self) -> Self::Output {
    let mut flags = FB_CLEAR;
    Self(bid128_add(self.0, rhs.0, RM_NEAREST_EVEN, &mut flags))
  }
}

impl std::ops::AddAssign<Self> for Decimal128 {
  ///
  fn add_assign(&mut self, rhs: Self) {
    let mut flags = FB_CLEAR;
    self.0 = bid128_add(self.0, rhs.0, RM_NEAREST_EVEN, &mut flags)
  }
}

impl std::ops::Sub<Self> for Decimal128 {
  type Output = Self;
  ///
  fn sub(self, rhs: Self) -> Self::Output {
    let mut flags = FB_CLEAR;
    Self(bid128_sub(self.0, rhs.0, RM_NEAREST_EVEN, &mut flags))
  }
}

impl std::ops::SubAssign<Self> for Decimal128 {
  ///
  fn sub_assign(&mut self, rhs: Self) {
    let mut flags = FB_CLEAR;
    self.0 = bid128_sub(self.0, rhs.0, RM_NEAREST_EVEN, &mut flags)
  }
}

impl std::ops::Mul<Self> for Decimal128 {
  type Output = Self;
  ///
  fn mul(self, rhs: Self) -> Self::Output {
    let mut flags = FB_CLEAR;
    Self(bid128_mul(self.0, rhs.0, RM_NEAREST_EVEN, &mut flags))
  }
}

impl std::ops::MulAssign<Self> for Decimal128 {
  ///
  fn mul_assign(&mut self, rhs: Self) {
    let mut flags = FB_CLEAR;
    self.0 = bid128_mul(self.0, rhs.0, RM_NEAREST_EVEN, &mut flags)
  }
}

impl std::ops::Div<Self> for Decimal128 {
  type Output = Self;
  ///
  fn div(self, rhs: Self) -> Self::Output {
    let mut flags = FB_CLEAR;
    Self(bid128_div(self.0, rhs.0, RM_NEAREST_EVEN, &mut flags))
  }
}

impl std::ops::DivAssign<Self> for Decimal128 {
  ///
  fn div_assign(&mut self, rhs: Self) {
    let mut flags = FB_CLEAR;
    self.0 = bid128_div(self.0, rhs.0, RM_NEAREST_EVEN, &mut flags)
  }
}

impl PartialEq<Self> for Decimal128 {
  ///
  fn eq(&self, rhs: &Self) -> bool {
    let mut flags = FB_CLEAR;
    bid128_quiet_equal(self.0, rhs.0, &mut flags)
  }
}

impl Eq for Decimal128 {}

impl PartialOrd<Self> for Decimal128 {
  ///
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let mut flags = FB_CLEAR;
    if bid128_quiet_equal(self.0, rhs.0, &mut flags) {
      return Some(Ordering::Equal);
    }
    flags = FB_CLEAR;
    if bid128_quiet_less(self.0, rhs.0, &mut flags) {
      return Some(Ordering::Less);
    }
    Some(Ordering::Greater)
  }
  ///
  fn lt(&self, rhs: &Self) -> bool {
    let mut flags = FB_CLEAR;
    bid128_quiet_less(self.0, rhs.0, &mut flags)
  }
  ///
  fn le(&self, rhs: &Self) -> bool {
    let mut flags = FB_CLEAR;
    bid128_quiet_less_equal(self.0, rhs.0, &mut flags)
  }
  ///
  fn gt(&self, rhs: &Self) -> bool {
    let mut flags = FB_CLEAR;
    bid128_quiet_greater(self.0, rhs.0, &mut flags)
  }
  ///
  fn ge(&self, rhs: &Self) -> bool {
    let mut flags = FB_CLEAR;
    bid128_quiet_greater_equal(self.0, rhs.0, &mut flags)
  }
}

impl Ord for Decimal128 {
  ///
  fn cmp(&self, rhs: &Self) -> Ordering {
    let mut flags = FB_CLEAR;
    if bid128_quiet_equal(self.0, rhs.0, &mut flags) {
      return Ordering::Equal;
    }
    flags = FB_CLEAR;
    if bid128_quiet_less(self.0, rhs.0, &mut flags) {
      return Ordering::Less;
    }
    Ordering::Greater
  }
  ///
  fn max(self, rhs: Self) -> Self
  where
    Self: Sized,
  {
    let mut flags = FB_CLEAR;
    Self(bid128_minnum(self.0, rhs.0, &mut flags))
  }
  ///
  fn min(self, rhs: Self) -> Self
  where
    Self: Sized,
  {
    let mut flags = FB_CLEAR;
    Self(bid128_minnum(self.0, rhs.0, &mut flags))
  }
}

impl From<&str> for Decimal128 {
  ///
  fn from(s: &str) -> Self {
    let mut flags = FB_CLEAR;
    Self(bid128_from_string(s, RM_NEAREST_EVEN, &mut flags))
  }
}

impl FromStr for Decimal128 {
  type Err = Infallible;
  ///
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(s.into())
  }
}

impl From<u8> for Decimal128 {
  /// Converts [Decimal128] from [u8].
  fn from(n: u8) -> Self {
    Self(bid128_from_uint32(n as u32))
  }
}

impl From<i8> for Decimal128 {
  /// Converts [Decimal128] from [i8].
  fn from(n: i8) -> Self {
    Self(bid128_from_int32(n as i32))
  }
}

impl From<u16> for Decimal128 {
  /// Converts [Decimal128] from [u16].
  fn from(n: u16) -> Self {
    Self(bid128_from_uint32(n as u32))
  }
}

impl From<i16> for Decimal128 {
  /// Converts [Decimal128] from [i16].
  fn from(n: i16) -> Self {
    Self(bid128_from_int32(n as i32))
  }
}

impl From<u32> for Decimal128 {
  /// Converts [Decimal128] from [u32].
  fn from(n: u32) -> Self {
    Self(bid128_from_uint32(n))
  }
}

impl From<i32> for Decimal128 {
  /// Converts [Decimal128] from [i32].
  fn from(n: i32) -> Self {
    Self(bid128_from_int32(n))
  }
}

impl From<u64> for Decimal128 {
  /// Converts [Decimal128] from [u64].
  fn from(n: u64) -> Self {
    Self(bid128_from_uint64(n))
  }
}

impl From<i64> for Decimal128 {
  /// Converts [Decimal128] from [i64].
  fn from(n: i64) -> Self {
    Self(bid128_from_int64(n))
  }
}

impl From<usize> for Decimal128 {
  /// Converts [Decimal128] from [usize].
  fn from(n: usize) -> Self {
    Self(bid128_from_uint64(n as u64))
  }
}

impl From<isize> for Decimal128 {
  /// Converts [Decimal128] from [isize].
  fn from(n: isize) -> Self {
    Self(bid128_from_int64(n as i64))
  }
}
