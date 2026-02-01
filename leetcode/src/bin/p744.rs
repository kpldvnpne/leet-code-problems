pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let letters = ["c","f","j"].iter().map(|s| s.chars().next().unwrap()).collect();
        let target = 'a';

        let expected = 'c';
        let actual = Solution::next_greatest_letter(letters, target);
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let letters = ["c","f","j"].iter().map(|s| s.chars().next().unwrap()).collect();
        let target = 'c';

        let expected = 'f';
        let actual = Solution::next_greatest_letter(letters, target);
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let letters = ["x","x","y","y"].iter().map(|s| s.chars().next().unwrap()).collect();
        let target = 'z';

        let expected = 'x';
        let actual = Solution::next_greatest_letter(letters, target);
        
        assert_eq!(expected, actual);
    }
}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut left = 0;
        let mut right = letters.len();

        while left != right && left < letters.len() {
            let mid = (left + right) / 2;
            if letters[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left == letters.len() {
            return letters[0];
        } else {
            return letters[left];
        }
    }
}