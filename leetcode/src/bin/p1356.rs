pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let arr = vec![0,1,2,3,4,5,6,7,8];
    let expected = vec![0,1,2,4,8,3,5,6,7];
    let actual = Solution::sort_by_bits(arr);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let arr = vec![1024,512,256,128,64,32,16,8,4,2,1];
    let expected = vec![1,2,4,8,16,32,64,128,256,512,1024];
    let actual = Solution::sort_by_bits(arr);

    assert_eq!(expected, actual);
  }
}

fn count_1bits(n: i32) -> u32 {
    let mut ans = 0;
    for i in 0..14 {
        if (n >> i) & 1 == 1 {
            ans += 1;
        }
    }
    return ans;
}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a, b| { count_1bits(*a).cmp(&count_1bits(*b)).then(a.cmp(b)) });
        arr
    }
}