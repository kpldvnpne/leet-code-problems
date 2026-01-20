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

fn sum_to_n(n: u32) -> u64 {
    n as u64 * (n + 1) as u64 / 2
}

fn main() {
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let [_n, k, q] = read_vector_line::<i32>()[..3] else { panic!() };
        let k = k as u32;

        let a = read_vector_line::<i32>();

        let mut total_ways: u64 = 0;
        let mut run = 0;
        for item in a {
            if item <= q {
                run += 1;
            } else {
                // Add run
                let n_ways = if run >= k { sum_to_n((run-(k-1)) as u32) } else {0};
                total_ways += n_ways;
                run = 0;
            }
        }

        let n_ways = if run >= k { sum_to_n((run-(k-1)) as u32) } else {0};
        total_ways += n_ways;

        println!("{}", total_ways);
    }
}