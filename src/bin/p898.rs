pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_1() {
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![0]), 1);
  }

  #[test]
  fn case_2() {
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
  }
}

use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
      return Solution::simple_way(&arr);
    }

    #[allow(dead_code)]
    fn simple_way(arr: &Vec<i32>) -> i32 {
      let mut set: HashSet<i32> = HashSet::new();
      let mut residues = vec![];

      for item in arr {
        let item = *item;
        set.insert(item);
        for residue_i in 0..residues.len() {
          let residue = residues[residue_i];
          let new_residue = residue | item;
          residues[residue_i] = new_residue;
          set.insert(new_residue);
        }
        residues.push(item);
      }

      return set.len() as i32;
    }
}
