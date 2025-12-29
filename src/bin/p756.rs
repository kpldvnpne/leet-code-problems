struct Solution;

fn main() {
  println!("Here");
  Solution::pyramid_transition("BCD".to_string(), vec!["BCC", "CDE", "CEA", "FFF"].iter().map(|it| it.to_string()).collect());
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let value = Solution::pyramid_transition("BCD".to_string(), vec!["BCC", "CDE", "CEA", "FFF"].iter().map(|it| it.to_string()).collect());
    assert!(value);
  }

  #[test]
  fn test2() {
    let value = Solution::pyramid_transition("AAAA".to_string(), vec![
      "AAB".to_string(),
      "AAC".to_string(),
      "BCD".to_string(),
      "BBE".to_string(),
      "DEF".to_string(),
    ]);
    assert!(!value);
  }

  #[test]
  fn test59() {
    let value = Solution::pyramid_transition("ABBBA".to_string(), vec![
      "ACA","ACF","ACE","ACD","ABA","ABF","ABE","ABD","FCA","FCF","FCE","FCD","FBA","FBF","FBE","FBD","ECA","ECF","ECE","ECD","EBA","EBF","EBE","EBD","DCA","DCF","DCE","DCD","DBA","DBF","DBE","DBD","CAA","CAF","CAE","CAD","CFA","CFF","CFE","CFD","CEA","CEF","CEE","CED","CDA","CDF","CDE","CDD","BAA","BAF","BAE","BAD","BFA","BFF","BFE","BFD","BEA","BEF","BEE","BED","BDA","BDF","BDE","BDD","CCA","CCF","CCE","CCD","CBA","CBF","CBE","CBD","BCA","BCF","BCE","BCD","BBA","BBF","BBE","BBD","CCC","CCB","CBC","CBB","BCC","BCB","BBC","BBB"
    ].iter().map(|it| it.to_string()).collect());
  }
}

use std::collections::HashMap;

fn sum_upto(n: u32) -> u32 {
  n * (n - 1) / 2
}

fn min_allowed_required(bottom: &str) -> u32 {
  let l = bottom.len();
  sum_upto((l - 1) as u32)
}

struct Checkpoint {
  pyramid: Vec<String>,
  row_i: usize,
  col_i: usize,
  next_layer: String,
  possible_next: String,
}

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
      if allowed.len() < min_allowed_required(&bottom) as usize {
        return false;
      }

      let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
      for item in allowed.iter() {
        let key: &str = &item[0..=1];
        let value: &str = &item[2..=2];

        map.entry(key).or_insert(vec![]).push(value);
      }

      let mut pyramid = vec![bottom];
      let mut row_i = 0;
      let mut col_i = 0;
      let mut next_layer: String = "".to_string();
      let mut checkpoints: Vec<Checkpoint> = vec![];
      loop {
        let key = &pyramid[row_i][col_i..=(col_i+1)];
        if let Some(possible_next) = map.get(key) {
          // Explore all possibilities
          if possible_next.len() > 1 {
            let mut new_checkpoints: Vec<Checkpoint> = possible_next[1..].iter().map(|n| {
              Checkpoint {
                pyramid: pyramid.clone(),
                row_i: row_i,
                col_i: col_i,
                next_layer: next_layer.clone(),
                possible_next: n.to_string(),
              }
            }).collect();
            checkpoints.append(&mut new_checkpoints);
          }

          next_layer.push_str(possible_next[0]);
          // for item in pyramid.iter() {
          //   println!("{}", item);
          // }
          // println!("Current bottom is {}", next_layer);
          // println!();
        } else {
          // println!("Could not find for {}", key);
          // Not possible
          if let Some(checkpoint) = checkpoints.pop() {
            pyramid = checkpoint.pyramid;
            row_i = checkpoint.row_i;
            col_i = checkpoint.col_i;
            next_layer = checkpoint.next_layer;
            next_layer.push_str(&checkpoint.possible_next);

            // println!("Trying next, current pyramid is");
            // for item in pyramid.iter() {
            //   println!("{}", item);
            // }
            // println!("Current bottom is {}", next_layer);
            // println!();
          } else {
            // println!("Returning false");
            return false;
          }
        }

        col_i += 1;
        if col_i + 1 == pyramid[row_i].len() {
          if col_i == 1 {
            println!("Returning true");
            return true;
          }
          col_i = 0;
          row_i += 1;
          pyramid.push(next_layer);
          next_layer = String::from("");
        }
      }
    }
}
