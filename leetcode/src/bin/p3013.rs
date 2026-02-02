pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![1, 3, 2, 6, 4, 2];
    let k = 3;
    let dist = 3;
    let expected = 5;
    let actual = Solution::minimum_cost(nums, k, dist);

    assert_eq!(actual, expected);
  }

  #[test]
  fn test2() {
    let nums = vec![10, 1, 2, 2, 2, 1];
    let k = 4;
    let dist = 3;
    let expected = 15;
    let actual = Solution::minimum_cost(nums, k, dist);

    assert_eq!(actual, expected);
  }

  #[test]
  fn test3() {
    let nums = vec![10, 8, 18, 9];
    let k = 3;
    let dist = 1;
    let expected = 36;
    let actual = Solution::minimum_cost(nums, k, dist);

    assert_eq!(actual, expected);
  }

  #[test]
  fn test4() {
    let nums = vec![36,28,42,36,39,13,24,3,32,16,11,43,21,40,34,49,29,20,34,34,8,3,41,6,46,5,35,5,47,2];
    println!("{}", nums.len());
    let k = 25;
    let dist = 26;
    let expected = 570;
    let actual = Solution::minimum_cost(nums, k, dist);

    assert_eq!(actual, expected);
  }
}

use std::collections::BTreeSet;

#[derive(Eq, PartialEq)]
struct TopNItem<T> {
  value: T,
  index: usize,
}

impl<T> Ord for TopNItem<T> where T: Ord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value).then(other.index.cmp(&self.index))
    }
}

impl<T> PartialOrd for TopNItem<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct TopNHolder<T, S> {
  _main_set: BTreeSet<TopNItem<T>>,
  _other_set: BTreeSet<TopNItem<T>>,
  _total: S,
  _n: usize,
}

impl TopNHolder<i32, i64> {
  fn new(n: usize) -> Self {
    Self {
      _main_set: BTreeSet::new(),
      _other_set: BTreeSet::new(),
      _total: 0,
      _n: n
    }
  }

  fn insert(&mut self, item: i32, position: usize) {
    let to_insert = TopNItem { value: item, index: position };
    if self._main_set.len() < self._n {
      self._main_set.insert(to_insert);
      self._total += item as i64;
      return;
    }
    if let Some(last) = self._main_set.last() && last > &to_insert {
      self._total = self._total - last.value as i64 + item as i64;
      let to_other = self._main_set.pop_last().unwrap();
      self._main_set.insert(to_insert);
      self._other_set.insert(to_other);
      return;
    }

    self._other_set.insert(to_insert);
  }

  fn remove(&mut self, item: i32, position: usize) {
    let to_remove = TopNItem {
      value: item,
      index: position
    };
    if self._main_set.contains(&to_remove) {
      self._total -= item as i64;
      self._main_set.remove(&to_remove);

      if self._main_set.len() < self._n {
        if let Some(other_first) = self._other_set.pop_first() {
          self._total += other_first.value as i64;
          self._main_set.insert(other_first);
        }
      }
    } else {
      self._other_set.remove(&to_remove);
    }
  }

  fn get_total(&mut self) -> i64 {
    self._total
  }
}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
      let k = k as usize;
      let dist = dist as usize;

      let first_elem = nums[0];
      let mut top_holder = TopNHolder::new(k - 1);
      for i in 1..=(dist+1) {
        top_holder.insert(nums[i], i);
      }

      let mut min_cost = top_holder.get_total();
      for i in dist+2..nums.len() {
        top_holder.remove(nums[i-dist-1], i-dist-1);
        top_holder.insert(nums[i], i);
        min_cost = min_cost.min(top_holder.get_total());
      }

      return first_elem as i64 + min_cost;
    }
}
