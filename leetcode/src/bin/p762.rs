pub struct Solution;

fn main() {}

fn count_bits(n: i32) -> u32 {
    let mut n_bits = 0;
    for i in 0..20 {
        let bit = (n >> i) & 1;
        if bit == 1 {
            n_bits += 1;
        }
    }
    return n_bits;
}

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let max_prime = 20;
        let mut is_prime = vec![true; max_prime + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..=max_prime {
            if is_prime[i] {
                let mut j = i * i;
                while j <= max_prime {
                    is_prime[j] = false;
                    j = j + i;
                }
            }
        }

        let mut ans: i32 = 0;
        for num in left..=right {
            let n_bits = count_bits(num);
            if is_prime[n_bits as usize] {
                ans += 1;
            }
        }

        return ans;
    }
}
