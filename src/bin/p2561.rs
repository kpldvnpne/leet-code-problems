pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_1() {
    assert_eq!(Solution::min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]), 1);
  }

  #[test]
  fn case_2() {
    assert_eq!(Solution::min_cost(vec![2, 3, 4, 1], vec![3, 2, 5, 1]), -1);
  }

  #[test]
  fn case_3() {
    assert_eq!(Solution::min_cost(vec![4, 4, 4, 4, 3], vec![5, 5, 5, 5, 3]), 8);
  }

  #[test]
  fn case_4() {
    Solution::min_cost(vec![84,80,43,8,80,88,43,14,100,88], vec![32,32,42,68,68,100,42,84,14,8]);
  }
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
struct PositionedItem {
  item: i32,
  position: usize,
}

impl PartialEq for PositionedItem {
  fn eq(&self, other: &Self) -> bool {
    self.item == other.item && self.position == other.position
  }
}

impl PartialOrd for PositionedItem {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match self.item.partial_cmp(&other.item) {
      Some(core::cmp::Ordering::Equal) => {}
      ord => return ord,
    }
    self.position.partial_cmp(&other.position)
  }
}

impl Eq for PositionedItem {}
impl Ord for PositionedItem {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match self.item.cmp(&other.item) {
      core::cmp::Ordering::Equal => {
        self.position.cmp(&other.position)
      }
      ord => return ord
    }
  }
}

struct Link {
  from_upper: BTreeSet<PositionedItem>,
  from_lower: BTreeSet<PositionedItem>,
}

impl Default for Link {
  fn default() -> Self {
    return Link{
      from_upper: BTreeSet::new(),
      from_lower: BTreeSet::new(),
    };
  }
}

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
      let mut links: BTreeMap<i32, Link> = BTreeMap::new();
      for i in 0..(basket1.len()) {
        let a = basket1[i];
        let b = basket2[i];

        if !links.contains_key(&a) {
          links.insert(a, Link::default());
        }

        if !links.contains_key(&b) {
          links.insert(b, Link::default());
        }

        let link: &mut Link = links.get_mut(&a).unwrap();
        link.from_upper.insert(PositionedItem{
          item: b,
          position: i,
        });

        let link: &mut Link = links.get_mut(&b).unwrap();
        link.from_lower.insert(PositionedItem {
          item: a,
          position: i,
        });
      }

      for (key, link) in links.iter() {
        let total_len = link.from_upper.len() + link.from_lower.len();
        if total_len % 2 != 0 {
          println!("{key}, {total_len}");
          return -1; // Not possible
        }
      }

      let mut new_basket1 = basket1.clone();
      let mut new_basket2 = basket2.clone();

      for key in links.keys() {
        let link = links.get_mut(key).unwrap();
        let diff = link.from_upper.len() as i32 - link.from_lower.len() as i32;
        let to_change_count = (diff.abs() / 2) as usize;

        let source;
        let dest;
        if diff > 0 {
          source = &mut link.from_upper;
          dest = &mut link.from_lower;
        } else {
          source = &mut link.from_lower;
          dest = &mut link.from_upper;
        }

        for item in source.clone().iter().rev().take(to_change_count) {
          source.remove(item);
          dest.insert(*item);

          let item_for_other = PositionedItem {
            item: *key,
            position: item.position
          };

          let other_linked = links.get_mut(&item.item).unwrap();
          let other_source;
          let other_dest;
          if diff > 0 {
            other_source = &mut other_linked.from_lower;
            other_dest = &mut other_linked.from_upper;
          } else {
            other_source = &mut other_linked.from_upper;
            other_dest = &mut other_linked.from_lower;
          }

          other_source.remove(&item_for_other);
          other_dest.insert(item_for_other);

          // Swap in the basket
          let position = item.position;
          let temp = new_basket1[position];
          new_basket1[position] = new_basket2[position];
          new_basket2[position] = temp;
        }
      }

      let mut answer: i64 = 0;
      for i in 0..new_basket1.len() {
        if new_basket1[i] != basket1[i] {
          // assert!(new_basket2[i] != new_basket2[i]);
          assert!(new_basket1[i] == basket2[i]);
          assert!(new_basket2[i] == basket1[i]);

          answer += std::cmp::min(new_basket1[i], new_basket2[i]) as i64;
        }
      }

      return answer;
    }
}
