fn main() {}

pub struct Solution;

use std::collections::HashMap;

fn to_hashmap(words: &Vec<String>) -> HashMap<&str, i32> {
  let mut map: HashMap<&str, i32> = HashMap::new();
  words.iter().for_each(|word| {
    let word: &str = word;
    map.insert(word, map.get(word).unwrap_or(&0) + 1);
  });

  return map;
}


fn substring(s: &str, start_pos: usize, len: usize) -> &str {
  return &s[start_pos..start_pos + len];
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
      assert!(words.len() >= 1);

      return Solution::method_2(&s, &words);
    }

    fn method_2(s: &String, words: &Vec<String>) -> Vec<i32> {
      let word_size = words[0].len();
      let total_size = word_size * words.len();

      if s.len() < total_size {
        return vec![];
      }

      let mut positions: Vec<i32> = vec![];
      let orig_map = to_hashmap(words);

      for offset in 0..word_size {
        let mut map: HashMap<&str, i32> = orig_map.clone();
        let mut last_word_pos = offset;

        for word_pos in (offset..(s.len() - word_size + 1)).step_by(word_size) {
          let word = substring(s, word_pos, word_size);

          while !map.contains_key(&word) && last_word_pos < word_pos {
            let word_to_put_back = substring(s, last_word_pos, word_size);
            last_word_pos += word_size;
            let count = map.get(&word_to_put_back).unwrap_or(&0);
            map.insert(word_to_put_back, count + 1);
          }

          if map.contains_key(&word) {
            let count = map.get(&word).unwrap_or(&0);
            let new_count = count - 1;
            if new_count == 0 {
              map.remove(&word);
            } else {
              map.insert(word,  new_count);
            }

            if map.is_empty() {
              positions.push(last_word_pos as i32);
              let word_to_put_back = substring(s, last_word_pos, word_size);
              last_word_pos += word_size;
              let count = map.get(&word_to_put_back).unwrap_or(&0);
              map.insert(word_to_put_back, count + 1);
            }
          } else {
            last_word_pos += word_size;
          }
        }
      }

      return positions;
    }

    #[allow(dead_code)]
    // This times out
    fn method_1(s: &String, words: &Vec<String>) -> Vec<i32> {
      let word_size = words[0].len();
      let total_len = word_size * words.len();
      let mut positions: Vec<i32> = vec![];

      for start_pos in 0..(s.len() - total_len + 1) {
        let mut map = to_hashmap(&words);
        for pos in (start_pos..(s.len() - word_size + 1)).step_by(word_size) {
          let slice: &str = substring(&s, pos, word_size);

          let count = map.get(&slice).unwrap_or(&0i32);
          if count > &0 {
            let new_count = count - 1;
            let slice: &str = substring(&s, pos, word_size);
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

    let mut expected_hashmap: HashMap<&str, i32> = HashMap::new ();
    expected_hashmap.insert(&words[0], 2i32);
    expected_hashmap.insert(&words[1], 1i32);

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

  #[test]
  fn it_works_when_s_smaller_than_words() {
    let s = "b".to_string();
    let words = vec!["bbb", "bbb"].iter().map(|w| { w.to_string() }).collect();

    assert_eq!(Solution::find_substring(s, words), vec![]);
  }

  #[test]
  fn it_does_not_time_out() {
    let s = "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab".to_string();

    let words: Vec<String> = vec!["ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba"].iter().map(|w| {w.to_string()}).collect();

    println!("{:?}", Solution::find_substring(s, words));
  }
}
