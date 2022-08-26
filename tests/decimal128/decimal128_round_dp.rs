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
