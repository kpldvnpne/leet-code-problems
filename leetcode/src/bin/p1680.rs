pub struct Solution;

fn main() {}

fn highest_bit_pos(n: i32, start: u32) -> u32 {
    for i in (0..=start).rev() {
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
        let mut start = 1;
        for i in 2..=n {
            let shift = highest_bit_pos(i, start+1);
            result = (result << shift) + i as u64;
            result = result % MOD;
            start = shift;
        }

        return result as i32;
    }
}
