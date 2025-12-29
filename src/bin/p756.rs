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

struct Checkpoint<'a> {
  current_layer: String,
  row_i: usize,
  col_i: usize,
  next_layer: String,
  possible_next: &'a[char],
}

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
      if allowed.len() < min_allowed_required(&bottom) as usize {
        return false;
      }

      let mut map: HashMap<(char, char), Vec<char>> = HashMap::new();
      for item in allowed.iter() {
        let first: char = item.chars().nth(0).unwrap();
        let second = item.chars().nth(1).unwrap();
        let third = item.chars().nth(2).unwrap();

        map.entry((first, second)).or_insert(vec![]).push(third);
      }

      let mut current_layer = bottom;
      let mut row_i = 0;
      let mut col_i = 0;
      let mut next_layer: String = "".to_string();
      let mut checkpoints: Vec<Checkpoint> = vec![];
      loop {
        let first = current_layer.chars().nth(col_i).unwrap();
        let second = current_layer.chars().nth(col_i+1).unwrap();
        if let Some(possible_next) = map.get(&(first, second)) {
          // Explore all possibilities
          if possible_next.len() > 1 {
            checkpoints.push(Checkpoint {
              current_layer: current_layer.clone(),
              row_i: row_i,
              col_i: col_i,
              next_layer: next_layer.clone(),
              possible_next: &possible_next[1..],
            });
          }

          next_layer.push(possible_next[0]);
          // for item in pyramid.iter() {
          //   println!("{}", item);
          // }
          // println!("Current bottom is {}", next_layer);
          // println!();
        } else {
          // println!("Could not find for {}", key);
          // Not possible
          if let Some(checkpoint) = checkpoints.pop() {
            current_layer = checkpoint.current_layer.clone();
            row_i = checkpoint.row_i;
            col_i = checkpoint.col_i;
            next_layer = checkpoint.next_layer.clone();
            if checkpoint.possible_next.len() > 1 {
              checkpoints.push(Checkpoint {
                possible_next: &checkpoint.possible_next[1..],
                ..checkpoint
              });
            }
            next_layer.push(checkpoint.possible_next[0]);

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
        if col_i + 1 == current_layer.len() {
          if col_i == 1 {
            // println!("Returning true");
            return true;
          }
          col_i = 0;
          row_i += 1;
          current_layer = next_layer;
          next_layer = String::from("");
        }
      }
    }
}
