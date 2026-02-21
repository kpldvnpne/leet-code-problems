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

use std::collections::HashMap;
fn get_mod(v: &Vec<u32>, n: u32) -> HashMap<u32, Vec<u32>> {
  let mut mods = HashMap::new();
  for &v_i in v {
    let rem = v_i % n;
    let e = mods.entry(rem).or_insert(vec![]);
    e.push(v_i);
  }
  return mods;
}

fn get_mod_count(v: &Vec<u32>, n: u32) -> HashMap<u32, usize> {
  let mut mods = HashMap::new();
  for &v_i in v {
    let rem = v_i % n;
    let e = mods.entry(rem).or_insert(0);
    *e += 1;
  }
  return mods;
}

fn main() {
  let t = read_val_line::<u32>();

  for _ in 0..t {
    let [n, x, y] = read_vector_line::<u32>()[..3] else { panic!() };
    let _n = n as usize;

    let a = read_vector_line::<u32>();

    let mod_x = get_mod(&a, x);

    let mut ans: u64 = 0;
    for (&key_x, v1) in mod_x.iter() {
      let other_key = x - key_x;
      if key_x > other_key {
        continue;
      }
      if key_x == 0 || key_x == other_key {
        let mod_y = get_mod_count(v1, y);
        for &l in mod_y.values() {
          if l > 0 {
            let l = l as u64;
            ans += l * (l - 1) / 2;
          }
        }
      } else if mod_x.contains_key(&other_key){
        let v2 = mod_x.get(&other_key).unwrap();
        let mod_y_v1 = get_mod_count(v1, y);
        let mod_y_v2 = get_mod_count(v2, y);

        for (key_v1, &l1) in mod_y_v1.iter() {
          if let Some(&l2) = mod_y_v2.get(key_v1) {
            let l1 = l1 as u64;
            let l2 = l2 as u64;
            // println!("{} {}", l1, l2);
            ans += l1 * l2;
          }
        }
      }
    }

    println!("{}", ans);
    
  }
}