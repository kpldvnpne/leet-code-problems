pub struct Solution;

fn main() {}

fn highest_bit_pos(n: i32) -> u32 {
    for i in (0..32).rev() {
        if (n >> i) & 1 == 1 {
            return i + 1;
        }
    }

    unreachable!();
}

const MOD: u64 = 1_000_000_000 + 7;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut result: u64 = 1;
        for i in 2..=n {
            let shift = highest_bit_pos(i);
            result = (result << shift) + i as u64;
            result = result % MOD;
        }

        return result as i32;
    }
}
