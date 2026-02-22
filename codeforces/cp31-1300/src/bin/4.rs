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

fn primes_upto_n(n: usize) -> Vec<u32> {
  let mut is_prime = vec![true; n as usize +1];
  is_prime[0] = false;
  is_prime[1] = false;
  let mut i = 2;
  while i * i <= n {
    if is_prime[i as usize] {
      let mut j = i * i;
      while j <= n {
        is_prime[j as usize] = false;
        j += i;
      }
    }
    i = i + 1;
  }

  let mut primes = vec![];
  for i in 2..=n {
    if is_prime[i as usize] {
      primes.push(i as u32);
    }
  }

  return primes;
}

fn main() {
  let primes_upto_1000 = primes_upto_n(1000);
  let n_primes = primes_upto_1000.len();

  let t = read_val_line::<u32>();
  for _ in 0..t {
    let n = read_val_line::<usize>();
    let a = read_vector_line::<u32>();

    use std::collections::HashMap;
    let mut divisors_count: HashMap<u32, u32> = HashMap::new();
    for &a_i in &a {
      let mut a_i = a_i;
      let mut p_i = 0;
      while p_i < n_primes && primes_upto_1000[p_i] <= a_i {
        let p = primes_upto_1000[p_i];
        while a_i % p == 0 {
          let count = divisors_count.entry(p).or_insert(0);
          *count += 1;
          a_i = a_i / p;
        }
        p_i += 1;
      }

      if a_i > 1 {
        let count = divisors_count.entry(a_i).or_insert(0);
        *count += 1;
      }
    }

    let mut possible = true;
    for &count in divisors_count.values() {
      if count % n as u32 != 0 {
        possible = false;
        break;
      }
    }

    if possible {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}