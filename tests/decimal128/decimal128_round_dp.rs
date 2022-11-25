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

use super::*;
use dfp_number::Decimal128;

#[test]
fn decimal128_round_dp_0001() {
  eqe("+16E-2", Decimal128::from("0.156").round_dp(2));
}

#[test]
fn decimal128_round_dp_0002() {
  eqe("+16E+2", Decimal128::from("1560").round_dp(-2));
}

#[test]
fn decimal128_round_dp_0003() {
  eqe("+24E+0", Decimal128::from("+23.5").round_dp(0));
}

#[test]
fn decimal128_round_dp_0004() {
  eqe("+24E+0", Decimal128::from("+24.5").round_dp(0));
}

#[test]
fn decimal128_round_dp_0005() {
  eqe("-24E+0", Decimal128::from("-23.5").round_dp(0));
}

#[test]
fn decimal128_round_dp_0006() {
  eqe("-24E+0", Decimal128::from("-24.5").round_dp(0));
}
