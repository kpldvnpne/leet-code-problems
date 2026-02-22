pub struct Solution;

fn main() {}

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut max = 0;
        let mut n = n;
        let mut last_1_pos = -1;
        let mut i = 0;
        while n > 0 {
            let bit = n & 1;
            n = n >> 1;
            if bit == 1 {
                if last_1_pos != -1 {
                    max = max.max(i - last_1_pos);
                }
                last_1_pos = i;

            }
            i = i + 1;
        }

        return max as i32;
    }
}
