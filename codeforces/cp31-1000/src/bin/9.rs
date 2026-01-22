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

fn get_smallest_divisor(n: u32) -> u32 {
    let max_divisor = (n as f64).sqrt() as u32;
    for divisor in 2..=max_divisor {
        if n % divisor == 0 {
            return divisor;
        }
    }
    return 1;
}

fn main() {
    let t = read_val_line::<usize>();
    for _ in 0..t {
        let n = read_val_line::<u32>();

        let smallest = get_smallest_divisor(n);
        if smallest == 1 {
            println!("{} {}", 1, n - 1);
        } else if smallest == 2 {
            println!("{} {}", n/2, n/2);
        } else {
            let biggest = n / smallest;
            let a = biggest;
            let b = n - biggest;

            println!("{} {}", a, b);
        }
    }
}

/*
Proof: Let a and b be u32 integers such that a + b == n.The max LCM one can achieve
while satisfying the answer is a = 1 and b = n - 1 which gives LCM = n - 1. Hence,
we need to see if we can achieve answer better than that.

The minimum LCM one can achieve is n / 2 when a = n / 2 and b = n / 2 when n is even.
For any value of a < n/2, b would be > n/2, which makes the LCM automatically  more.

When n is odd, a = n / 2 and b = n / 2 + 1, and two numbers that are apart by 1 is
never co-prime, which leads our answer to be (n^2 + 2n)/4. This is worse than the n-1
when n >= 4. Hence, the answer is somewhere in between.

If x is the greatest divisor of n that is < n, then, (n - x) and x would be the answer, because 
(n-x) would be the LCM which is already smaller than n - 1.
*/