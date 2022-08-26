use dfp_number::Decimal128;

mod decimal128_display;
mod decimal128_new;
mod decimal128_round_dp;

fn eqe(expected: &str, actual: Decimal128) {
  assert_eq!(expected, format!("{:?}", actual));
}
