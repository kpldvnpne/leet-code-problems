pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let s = "aababbab".to_string();
    let expected = 2;
    let actual = Solution::minimum_deletions(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s = "bbaaaaabb".to_string();
    let expected = 2;
    let actual = Solution::minimum_deletions(s);

    assert_eq!(expected, actual);
  }
}

impl Solution {
  pub fn minimum_deletions(s: String) -> i32 {
    let n = s.len();
    let mut n_bs = vec![0; n+1];
    let mut n_as = vec![0; n+1];

    for (i, c) in s.chars().enumerate() {
      if c == 'a' {
        n_as[i+1] = n_as[i] + 1;
        n_bs[i+1] = n_bs[i];
      } else if c == 'b' {
        n_bs[i+1] = n_bs[i] + 1;
        n_as[i+1] = n_as[i];
      } else {
        unreachable!();
      }
    }

    // println!("{:?}", n_as);

    let mut min_del = n - 1;
    let last_n_as = n_as[n];
    for i in 0..n+1 {
      let n_del_b = n_bs[i];
      let n_del_a = last_n_as - n_as[i];
      let n_del = n_del_a + n_del_b;
      min_del = min_del.min(n_del);
    }

    return min_del as i32;
  }
}