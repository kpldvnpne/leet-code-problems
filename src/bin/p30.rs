fn main() {}

pub struct Solution;

use std::collections::HashSet;
use std::collections::HashMap;

fn to_hashset(words: &Vec<String>) -> HashSet<&String> {
  let mut set = HashSet::new();
  words.iter().for_each(|word| {
    set.insert(word);
  });

  return set;
}

fn to_hashmap(words: &Vec<String>) -> HashMap<String, i32> {
  let mut map = HashMap::new();
  words.iter().for_each(|word| {
    map.insert(word.into(), map.get(word).unwrap_or(&0) + 1);
  });

  return map;
}


fn substring(s: &String, start_pos: usize, len: usize) -> String {
  return s.chars().skip(start_pos).take(len).collect();
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
      assert!(words.len() >= 1);

      let word_size = words[0].len();
      let total_len = word_size * words.len();
      let mut positions: Vec<i32> = vec![];

      for start_pos in 0..(s.len() - total_len + 1) {
        let mut map = to_hashmap(&words);
        for pos in (start_pos..(s.len() - word_size + 1)).step_by(word_size) {
          let slice: String = substring(&s, pos, word_size);

          let count = map.get(&slice).unwrap_or(&0i32);
          if count > &0 {
            let new_count = count - 1;
            let slice: String = substring(&s, pos, word_size);
            if new_count == 0 {
              map.remove(&slice);
            } else {
              map.insert(slice, new_count);
            }
          } else {
            break;
          }
        }

        if map.is_empty() {
          positions.push(start_pos as i32);
        }
      }

      return positions;
    }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_can_substring() {
    let s: String = "barfoo".to_string();

    assert_eq!(substring(&s, 3, 3), "foo".to_string());
  }

  #[test]
  fn it_can_convert_to_map() {
    let words = vec!["abc".to_string(), "def".to_string(), "abc".to_string()];

    let mut expected_hashmap = HashMap::new ();
    expected_hashmap.insert(words[0].clone(), 2i32);
    expected_hashmap.insert(words[1].clone(), 1i32);

    assert_eq!(to_hashmap(&words), expected_hashmap);
  }

  #[test]
  fn it_finds_substrings() {
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];

    assert_eq!(Solution::find_substring(s, words), vec![0, 9]);
  }

  #[test]
  fn it_works_for_duplicate_words() {
    let s = "barfoothebarfoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string(), "bar".to_string()];


    assert_eq!(Solution::find_substring(s, words), vec![9]);
  }

  #[test]
  fn it_works_when_at_last() {
    let s = "wordgoodgoodgoodbestword".to_string();
    let words: Vec<String> = vec!["word","good","best","good"].iter().map(|w| { w.to_string() }).collect();

    // println!("{:?}", Solution::find_substring(s, words));

    assert_eq!(Solution::find_substring(s, words), vec![8]);
  }
}
