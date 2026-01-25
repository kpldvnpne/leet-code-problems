fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
#[allow(dead_code)]
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

#[allow(dead_code)]
fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

#[allow(dead_code)]
fn read_string_array() -> Vec<String> {
    read_line().trim().split_ascii_whitespace().map(|it| it.trim().to_string()).collect()
}

fn get_number(pair: (u8, u8)) -> u8 {
  pair.0 * 10 + pair.1
}

fn main() {
  let mut ans: u32 = 0;
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    // println!("{}", line);

    let line_digits: Vec<u8> = line.as_bytes().into_iter().map(|it| it - '0' as u8).collect();
    let mut digits = (line_digits[0], line_digits[1]);
    for i in 2..line_digits.len() {
      let new_digit_1 = (digits.0, line_digits[i]);
      let new_digit_2 = (digits.1, line_digits[i]);

      if get_number(new_digit_1) > get_number(digits) {
        digits = new_digit_1;
      }

      if get_number(new_digit_2) > get_number(digits) {
        digits = new_digit_2;
      }
    }

    let max_battery_capacity = get_number(digits);

    ans += max_battery_capacity as u32;
  }

  println!("{}", ans);
}
