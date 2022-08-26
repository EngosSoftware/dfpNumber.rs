use super::*;
use dfp_number::Decimal128;

#[test]
fn decimal128_new_0001() {
  eqe("+123456E-2", Decimal128::new(123456, 2));
}

#[test]
fn decimal128_new_0002() {
  eqe("+123456E+2", Decimal128::new(123456, -2));
}

#[test]
fn decimal128_new_0003() {
  eqe("+123456E-7", Decimal128::new(123456, 7));
}

#[test]
fn decimal128_new_0004() {
  eqe("+123000E+0", Decimal128::new(123000, 0));
}
