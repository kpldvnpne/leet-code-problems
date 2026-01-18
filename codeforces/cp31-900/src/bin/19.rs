fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

// 1: -1
// 2: 1
// 3: 4
// 4: 0
// n (divisible by 4): 0
// n + 1: 0 - n - 1 = -n - 1
// n + 2: -n - 1 + n + 2 = 1
// n + 3: 1 + n + 3 = n + 4
// n + 4: n + 4 - n - 4 = 0

fn get_delta(n: u64) -> i64 {
    let base = n / 4 * 4;
    match n % 4 {
        0 => 0,
        1 => -1 - base as i64,
        2 => 1,
        3 => base as i64 + 4,
        _ => unreachable!()
    }
}

fn main() {
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let [x, n] = read_vector_line::<i64>()[..2] else { panic!() };
        let delta = get_delta(n as u64);
        let ans = if x % 2 == 0 {
            x + delta
        } else {
            x - delta
        };

        println!("{ans}");
    }
}
