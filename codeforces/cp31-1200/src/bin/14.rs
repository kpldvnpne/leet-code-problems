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

fn rot90(dx: usize, dy: usize, ox: usize, oy: usize) -> (usize, usize) {
  (ox + dy, oy - dx)
}

fn rot180(dx: usize, dy: usize, ox: usize, oy: usize) -> (usize, usize) {
  (ox + dx, oy + dy)
}

fn rot270(dx: usize, dy: usize, ox: usize, oy: usize) -> (usize, usize) {
  (ox - dy, oy + dx)
}

fn count_ops(a: &Vec<Vec<u8>>, i: usize, j: usize, mid: usize, n: usize) -> usize {
  let res = if n % 2 == 0 { 1 } else { 0 };
  let dx = mid - i;
  let dy = mid - j;

  let indices = [(i, j), rot90(dx, dy, mid-res, mid), rot180(dx, dy, mid-res, mid-res), rot270(dx, dy, mid, mid-res)];
  // println!("{:?}", indices);

  let items = indices.map(|(i, j)| a[i][j]);
  // println!("{:?}", items);

  let count_1 = items.iter().filter(|a| **a == 1).count();
  let count_0 = items.iter().filter(|a| **a == 0).count();

  // println!("{} {}", count_0, count_1);

  return count_1.min(count_0);
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let mut a = vec![];
    for _ in 0..n {
      let a_i = read_line().trim().bytes().into_iter().map(|a| a - '0' as u8).collect::<Vec<u8>>();
      a.push(a_i);
    }

    // println!("{:#?}", a);

    let mid = n / 2;
    let mut total_ops = 0;
    for i in 0..mid {
      for j in 0..mid {
        
        total_ops += count_ops(&a, i, j, mid, n);
        // println!("Debug: {}", total_ops);
      }
    }

    if n % 2 == 1 {
      for i in 0..mid {
        let j = mid;
        total_ops += count_ops(&a, i, j, mid, n);
        // println!("Debug: {}", total_ops);
      }
    }

    println!("{}", total_ops);
  }
}