fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::str::FromStr;
use std::fmt::Debug;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn read_int_array<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.parse().unwrap()).collect()
}

use std::collections::BinaryHeap;
use core::cmp::Reverse;

fn main() {
  let t: i32 = read_val_line();
  for _ in 0..t {
    let n_m_k = read_int_array::<i64>();
    let (n, m, k) = (n_m_k[0], n_m_k[1], n_m_k[2]);
    let m = m as usize;

    // println!("n={n}, m={m}, k={k}");

    let a = read_int_array::<i32>();

    let mut a_map: Vec<(i32, Vec<i32>)> = vec![(0, vec![]); m+1];
    for a_item in &a {
      a_map[*a_item as usize].0 += 1;
    }

    let mut rem_k = k;
    for _ in 0..n {
      let x_y_z = read_int_array::<i32>();
      let (x, y, z) = (x_y_z[0], x_y_z[1], x_y_z[2]);

      rem_k -= y as i64;
      let rem_price = z - y;
      a_map[x as usize].1.push(rem_price);
    }

    // Remove using gifts
    let mut n_happy = 0;
    let mut prices_max_heap = BinaryHeap::new();
    for a_i in 1..(m+1) {
      let list = &a_map[a_i].1;
      if !list.is_empty() {
        list.iter().for_each(|it| prices_max_heap.push(*it));
      }

      let count = a_map[a_i].0;
      if count <= 0 {
        continue;
      }

      // Remove `count` elements from either current or previous
      let mut removed = 0;
      while removed < count && !prices_max_heap.is_empty() {
        prices_max_heap.pop();
        removed += 1;
        // println!("Removed because of gift box of beauty {}", a_i);
      }

      n_happy += removed;
    }

    // Now, use the rem_k to remove more
    let mut prices_min_heap = BinaryHeap::new();
    for price in prices_max_heap {
      prices_min_heap.push(Reverse(price));
    }

    while !prices_min_heap.is_empty() {
      if let Some(price) = prices_min_heap.pop() {
        let price = price.0 as i64;
        if price > rem_k {
          break;
        } else {
          rem_k -= price;
          n_happy += 1;
          // println!("Happy because we could pay price {price}");
        }
      }
    }

    println!("{n_happy}");
  }
}
