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
      return Solution::second_way(&arr);
    }

    fn second_way(arr: &Vec<i32>) -> i32 {
      let mut set: HashSet<i32> = HashSet::new();
      let mut residues = vec![];

      let mut residue_start = 0;
      for item in arr {
        let item = *item;
        set.insert(item);
        for residue_i in residue_start..residues.len() {
          let residue = residues[residue_i];
          let new_residue = residue | item;
          residues[residue_i] = new_residue;
          set.insert(new_residue);

          if residue_i > residue_start {
            let prev_residue = residues[residue_i - 1] ;
            if new_residue == prev_residue {
              residue_start = residue_i;
            }
          }
        }
        residues.push(item);
      }

      return set.len() as i32;
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
