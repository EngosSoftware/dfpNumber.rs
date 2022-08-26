use dfp_number::Decimal128;

#[test]
fn decimal128_new_0001() {
  assert_eq!("+123456E-2", Decimal128::new(123456, 2).to_string());
}

#[test]
fn decimal128_new_0002() {
  assert_eq!("+123456E+2", Decimal128::new(123456, -2).to_string());
}

#[test]
fn decimal128_new_0003() {
  assert_eq!("+123456E-7", Decimal128::new(123456, 7).to_string());
}

#[test]
fn decimal128_new_0004() {
  assert_eq!("+123000E+0", Decimal128::new(123000, 0).to_string());
}
