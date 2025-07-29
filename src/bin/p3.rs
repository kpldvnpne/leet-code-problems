// Better than 100% of the code out there.
pub struct Solution;

fn main() {}

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_c_loc: HashMap<u8, usize> = HashMap::new();

        let mut max_len: i32 = 0;
        let mut start = 0;
        let mut len = 0;
        let s = s.as_bytes();
        for s_i in 0..s.len() {
            let c = s[s_i];
            let appeared_before: bool = last_c_loc.contains_key(&c) && *last_c_loc.get(&c).unwrap() >= start;
            if !appeared_before {
                len += 1;
                if max_len < len {
                    max_len = len;
                }

                last_c_loc.insert(c, s_i);
            } else {
                start = *last_c_loc.get(&c).unwrap();
                last_c_loc.insert(c, s_i);
                len = (s_i - start) as i32;
            }
        }

        return max_len;
    }
}
