pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    println!("{}", Solution::count_prime_set_bits(842, 888));
  }
}

#[allow(non_snake_case)]
fn nCr(n: u32, r: u32) -> u32 {
    if r > n { return 0 };
    if r == n || r == 0 { return 1 };
    let r = if r < n - r { r } else { n - r };
    let mut ans: u64 = 1;
    for r_i in 0..r {
        ans = ans * (n - r_i) as u64 / (r_i + 1) as u64;
    }
    return ans as u32;
}

fn n_primes_upto(n: i32, primes: &[u32]) -> u32 {
    let mut n_1s = 0;
    let mut ans = 0;
    for len in (0..=20).rev() {
        if (n >> len) & 1 == 1 {
            for &prime in primes {
                if n_1s > prime {
                    continue;
                }
                let r = prime - n_1s;
                ans += nCr(len, r);
            }

            n_1s += 1;
        }
    }

    if primes.contains(&n_1s) {
      ans += 1;
    }
    return ans;
}


impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
      let all_primes = [2, 3, 5, 7, 11, 13, 17, 19];

      return n_primes_upto(right, &all_primes) as i32 - n_primes_upto(left - 1, &all_primes) as i32;
    }
}
