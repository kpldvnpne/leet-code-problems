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

fn main() {
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let [a, b, c] = read_vector_line::<i32>()[..3] else { panic!() };

        let expected_a = b - (c - b);
        let expected_b = (a + c) / 2;
        let expected_c = b + (b - a);

        let a_possible = expected_a > 0 && expected_a % a == 0;
        let b_possible = (a + c) % 2 == 0 && expected_b > 0 && expected_b % b == 0;
        let c_possible = expected_c > 0 && expected_c % c == 0;

        if a_possible || b_possible || c_possible {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}