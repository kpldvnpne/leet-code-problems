pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let s = "(()".to_string();
        let expected = 2;
        let actual = Solution::longest_valid_parentheses(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let s = ")()())".to_string();
        let expected = 4;
        let actual = Solution::longest_valid_parentheses(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let s = "".to_string();
        let expected = 0;
        let actual = Solution::longest_valid_parentheses(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test105() {
        let s = "()(())".to_string();
        let expected = 6;
        let actual = Solution::longest_valid_parentheses(s);
        assert_eq!(expected, actual);
    }
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let mut stack = vec![];
        let mut valid_subs = vec![];
        for (i, c) in s.iter().enumerate() {
            if *c == '(' as u8 {
                stack.push(i);
            } else {
                if let Some(last) = stack.pop() {
                    if valid_subs.is_empty() {
                        valid_subs.push((last, i));
                    } else {
                        let length = valid_subs.len();
                        let (before_last_arr, last_arr) = valid_subs.split_at_mut(length - 1);
                        let last_sub = &mut last_arr[0];
                        if last_sub.1 + 1 == last {
                            last_sub.1 = i;
                        } else if last_sub.0 == last + 1 {
                            last_sub.0 = last;
                            last_sub.1 = i;

                            if !before_last_arr.is_empty() {
                                let second_last_sub = &mut before_last_arr[before_last_arr.len() - 1];
                                if second_last_sub.1 + 1 == last_sub.0 {
                                    second_last_sub.1 = last_sub.1;
                                    valid_subs.pop();
                                }
                            }
                        } else {
                            valid_subs.push((last, i));
                        }
                    }
                }
            }
        }

        let mut max_valid_len = 0;
        for sub in valid_subs {
            let length = (sub.1 - sub.0 + 1) as i32;
            max_valid_len = std::cmp::max(length, max_valid_len);
        }

        return max_valid_len;
    }
}