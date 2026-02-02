pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let source = "abcd".to_string();
    let target = "acbe".to_string();
    let original = vec!["a","b","c","c","e","d"].iter().map(|it| it.chars().next().unwrap()).collect();
    let changed = vec!["b","c","b","e","b","e"].iter().map(|it| it.chars().next().unwrap()).collect();
    let cost = vec![2,5,5,1,2,20];

    let expected = 28;
    let actual = Solution::minimum_cost(source, target, original, changed, cost);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let source = "aaaa".to_string();
    let target = "bbbb".to_string();
    let original = vec!["a", "c"].iter().map(|it| it.chars().next().unwrap()).collect();
    let changed = vec!["c", "b"].iter().map(|it| it.chars().next().unwrap()).collect();
    let cost = vec![1, 2];

    let expected = 12;
    let actual = Solution::minimum_cost(source, target, original, changed, cost);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let source = "abcd".to_string();
    let target = "abce".to_string();
    let original = vec!["a"].iter().map(|it| it.chars().next().unwrap()).collect();
    let changed = vec!["e"].iter().map(|it| it.chars().next().unwrap()).collect();
    let cost = vec![10000];

    let expected = -1;
    let actual = Solution::minimum_cost(source, target, original, changed, cost);

    assert_eq!(expected, actual);
  }
}

fn to_index(c: char) -> usize {
  (c as u8 - 'a' as u8) as usize
}

impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {

      let mut map = vec![vec![0; 26]; 26];
      for ((a, b), c) in original.into_iter().zip(changed).zip(cost) {
        let a = to_index(a);
        let b = to_index(b);
        if map[a][b] == 0 {
          map[a][b] = c;
        } else {
          map[a][b] = map[a][b].min(c);
        }
      }

      // println!("Before:");
      // for row in &map {
      //   println!("{:?}", row);
      // }

      for _ in 0..26 {
        let mut changed = false;
        for source in 0..26 {
          for intermediate in 0..26 {
            if source == intermediate {
              continue;
            }
            if map[source][intermediate] > 0 {
              for destination in 0..26 {
                if source == destination {
                  continue;
                }
                if map[intermediate][destination] > 0 {
                  let new_cost = map[source][intermediate] + map[intermediate][destination];
                  if map[source][destination] > new_cost || map[source][destination] == 0 {
                    map[source][destination] = new_cost;
                    changed = true;
                  }
                }
              }
            }
          }
        }

        if !changed {
          break;
        }

      }

      // println!("After:");
      // for row in &map {
      //   println!("{:?}", row);
      // }

      let mut ans = 0;
      for (s, t) in source.chars().zip(target.chars()) {
        if s != t {
          let entry = map[to_index(s)][to_index(t)];
          if entry > 0 {
            ans += entry as i64;
          } else {
            return -1;
          }
        }
      }

      return ans;
    }
}
