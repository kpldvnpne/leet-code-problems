pub struct Solution;

fn main() {}

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let s = s.as_bytes();

        let mut carry = 0;
        let mut ans = 0;
        for (i, bit) in s.iter().enumerate().rev() {
            let bit = bit - '0' as u8;
            let sum = carry + bit;
            if sum == 1 {
                if i != 0 {
                    ans += 2;
                    carry = 1;
                }
            } else {
                ans += 1;
                carry = sum >> 1;
            }
        }

        return ans;
    }
}
