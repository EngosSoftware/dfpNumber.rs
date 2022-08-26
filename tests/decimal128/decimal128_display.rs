use dfp_number::Decimal128;

#[test]
fn decimal128_new_0001() {
  assert_eq!("1.2", format!("{}", Decimal128::new(120, 2)));
}

#[test]
fn decimal128_new_0001_1() {
  assert_eq!("0.0000012", format!("{}", Decimal128::new(120, 8)));
}

#[test]
fn decimal128_new_0001_2() {
  assert_eq!("0.12", format!("{}", Decimal128::new(120, 3)));
}

#[test]
fn decimal128_new_0001_3() {
  assert_eq!("12000", format!("{}", Decimal128::new(120, -2)));
}

#[test]
fn decimal128_new_0002() {
  assert_eq!("-1.2", format!("{:20}", Decimal128::new(-12, 1)));
}

#[test]
fn decimal128_new_0003() {
  assert_eq!("1.2", format!("{:20.2}", Decimal128::new(12, 1)));
}

#[test]
fn decimal128_new_0004() {
  assert_eq!("1.2", format!("{:>20.2}", Decimal128::new(12, 1)));
}

#[test]
fn decimal128_new_0005() {
  assert_eq!("1.2", format!("{:<20.2}", Decimal128::new(12, 1)));
}

#[test]
fn decimal128_new_0006() {
  assert_eq!("1.2", format!("{:^20.2}", Decimal128::new(12, 1)));
}

#[test]
fn decimal128_new_0007() {
  assert_eq!("1.2", format!("{:G^20.2}", Decimal128::new(12, 1)));
}

#[test]
fn decimal128_new_0008() {
  assert_eq!(
    "0.00000000000000000001",
    format!("{:G^20.2}", Decimal128::from("1000.0000000e-23"))
  );
}

#[test]
fn decimal128_new_0009() {
  assert_eq!("+Inf", format!("{}", Decimal128::from("Inf")));
}

#[test]
fn decimal128_new_0010() {
  assert_eq!("+Inf", format!("{}", Decimal128::from("Infinity")));
}

#[test]
fn decimal128_new_0011() {
  assert_eq!("+Inf", format!("{}", Decimal128::from("+Inf")));
}

#[test]
fn decimal128_new_0012() {
  assert_eq!("-Inf", format!("{}", Decimal128::from("-Inf")));
}

#[test]
fn decimal128_new_0013() {
  assert_eq!("-Inf", format!("{}", Decimal128::from("-Infinity")));
}

#[test]
fn decimal128_new_0014() {
  assert_eq!("+NaN", format!("{}", Decimal128::from("NaN")));
}

#[test]
fn decimal128_new_0015() {
  assert_eq!("+NaN", format!("{}", Decimal128::from("+NaN")));
}

#[test]
fn decimal128_new_0016() {
  assert_eq!("-NaN", format!("{}", Decimal128::from("-NaN")));
}

#[test]
fn decimal128_new_0017() {
  assert_eq!("-NaN", format!("{}", Decimal128::from("-NAN")));
}

#[test]
fn decimal128_new_0018() {
  assert_eq!("-NaN", format!("{}", Decimal128::from("-nan")));
}

#[test]
fn decimal128_new_0019() {
  assert_eq!("-SNaN", format!("{}", Decimal128::from("-snan")));
}

#[test]
fn decimal128_new_0020() {
  assert_eq!("+SNaN", format!("{}", Decimal128::from("snan")));
}
