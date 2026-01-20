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
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let n = read_val_line::<usize>();
        let s = read_line();
        let s = s.trim().as_bytes();

        use std::collections::HashSet;
        let mut present_chars = HashSet::new();
        let mut distinct_chars = vec![0; n];
        for i in 0..n {
            let c = s[i];
            present_chars.insert(c);

            distinct_chars[i] = present_chars.len();
        }
    
        let mut present_chars = HashSet::new();
        let mut max_distinct = distinct_chars[n-1];
        for i in (0..n).rev() {
            let c = s[i];
            present_chars.insert(c);

            let distinct = present_chars.len() + if i > 0 { distinct_chars[i-1] } else { 0 };
            max_distinct = std::cmp::max(distinct, max_distinct);
        }

        let ans = max_distinct;

        println!("{ans}");
    }
}