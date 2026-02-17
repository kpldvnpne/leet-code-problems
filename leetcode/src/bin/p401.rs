pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let turned_on = 1;
    let expected = ["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"].iter().map(|it| it.to_string()).collect::<Vec<_>>();
    let actual = Solution::read_binary_watch(turned_on);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let turned_on = 9;
    let expected: Vec<String> = vec![];
    let actual = Solution::read_binary_watch(turned_on);

    assert_eq!(expected, actual);
  }
}

fn get_permutation(size: usize, required_on: usize) -> Vec<Vec<u8>> {
  assert!(required_on <= size, "{} not le {}", size, required_on);
  if required_on == 0 {
    return vec![vec![0u8; size]];
  }
  if size == required_on {
    return vec![vec![1u8; size]];
  }

  let with_this_off = get_permutation(size - 1, required_on);
  let with_this_on = get_permutation(size - 1, required_on - 1);
  let mut to_return = Vec::with_capacity(with_this_off.len() + with_this_on.len());

  for mut v in with_this_off {
    v.push(0);
    to_return.push(v);
  }

  for mut v in with_this_on {
    v.push(1);
    to_return.push(v);
  }
  return to_return;
}

fn get_hour(perm: &[u8]) -> Option<u8> {
  assert!(perm.len() == 4);
  let mut hour = 0;
  for item in perm {
    hour = hour * 2 + item;
  }

  if hour < 12 {
    return Some(hour);
  } else {
    return None;
  }
}

fn get_minute(perm: &[u8]) ->  Option<u8> {
  assert!(perm.len() == 6);
  let mut minute = 0;
  for item in perm {
    minute = minute * 2 + item;
  }

  if minute < 60 {
    return Some(minute);
  } else {
    return None;
  }
}

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
      if turned_on > 8 {
        return vec![];
      }

      let permutations = get_permutation(10, turned_on as usize);

      // println!("{:?}", permutations);

      let mut ans = vec![];
      for mut perm in permutations {
        perm.reverse();
        if let Some(hour) = get_hour(&perm[..4]) {
          if let Some(min) = get_minute(&perm[4..perm.len()]) {
            let time = format!("{}:{:02}", hour, min);
            ans.push(time);
          }
        }

      }

      return ans;
    }
}
