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
    let [n, d] = read_vector_line::<usize>()[0..2] else { panic!() };
    let mut p= read_vector_line::<usize>();

    p.sort_by(|a, b| b.cmp(&a)); // Sort reverse
    let mut i: usize = 0; 
    let mut used: usize = 0;
    let mut n_teams = 0;
    while i < n && used <= n {
        let needed = (d + p[i]) / p[i];

        used = used + needed;

        if used <= n {
            n_teams += 1;
        }

        i = i + 1;
    }

    println!("{}", n_teams);
}