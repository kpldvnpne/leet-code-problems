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
        let [n, k] = read_vector_line::<u32>()[..2] else { panic!() };
        let n = n as usize;
        let a = read_vector_line::<u32>();

        let mut arr = Vec::with_capacity(n);
        for i in 0..n {
            let item = a[i];
            let rem = if item % k == 0 { k } else { item % k };
            arr.push((rem, i));
        }

        arr.sort_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));

        for (_, i) in arr {
            print!("{} ", i+1);
        }
        println!("");
    }
}