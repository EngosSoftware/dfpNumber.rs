use dfp_number::Decimal128;

#[test]
fn decimal128_round_dp_0001() {
  assert_eq!("+16E-2", Decimal128::from("0.156").round_dp(2).to_string());
}

#[test]
fn decimal128_round_dp_0002() {
  assert_eq!("+16E+2", Decimal128::from("1560").round_dp(-2).to_string());
}

#[test]
fn decimal128_round_dp_0003() {
  assert_eq!("+24E+0", Decimal128::from("+23.5").round_dp(0).to_string());
}

#[test]
fn decimal128_round_dp_0004() {
  assert_eq!("+24E+0", Decimal128::from("+24.5").round_dp(0).to_string());
}

#[test]
fn decimal128_round_dp_0005() {
  assert_eq!("-24E+0", Decimal128::from("-23.5").round_dp(0).to_string());
}

#[test]
fn decimal128_round_dp_0006() {
  assert_eq!("-24E+0", Decimal128::from("-24.5").round_dp(0).to_string());
}
