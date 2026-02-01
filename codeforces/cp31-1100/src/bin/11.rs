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

fn main() {
    let t = read_val_line::<u32>();
    for _ in 0..t {
        let [n, k] = read_vector_line::<usize>()[..2] else { panic!() };
        let mut a = read_vector_line::<u64>();


        a.sort();
        let mut min_elems: Vec<u64> = Vec::with_capacity(k);
        min_elems.push(0);
        let mut max_elems: Vec<u64> = Vec::with_capacity(k);
        max_elems.push(0);
        let mut min_sum: u64 = 0;
        let mut max_sum: u64 = 0;
        for i in 0..k {
            min_sum += a[2*i] as u64  + a[2*i+1] as u64;
            max_sum += a[n-1-i] as u64;
            min_elems.push(min_sum);
            max_elems.push(max_sum);
        }

        let sum = a.into_iter().sum::<u64>();
        let rm_sum = min_elems.iter().zip(max_elems.iter().rev()).map(|it| it.0 + it.1).min().unwrap();
        let ans = sum - rm_sum;
        println!("{}", ans);
    }
}