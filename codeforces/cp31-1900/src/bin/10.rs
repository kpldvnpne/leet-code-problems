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
        let [n, q] = read_vector_line::<usize>()[..2] else { panic!() };
        let a = read_vector_line::<u32>();

        let mut prefix_sums = vec![0u64; n];
        prefix_sums[0] = a[0] as u64;
        for i in 1..n {
            prefix_sums[i] = prefix_sums[i-1] + a[i] as u64;
        }

        let total_sum = prefix_sums[n-1];

        for _ in 0..q {
            let [l, r, k] = read_vector_line::<usize>()[..3] else { panic!() };
            let k = k as u64;

            let sum_to_before_l = if l >= 2 { prefix_sums[l-2] } else { 0 };
            let sum_to_r = prefix_sums[r-1];
            let l_to_r_sum = sum_to_r - sum_to_before_l;

            let new_sum = total_sum - l_to_r_sum + k * (r - l + 1) as u64;

            if new_sum % 2 != 0 {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}