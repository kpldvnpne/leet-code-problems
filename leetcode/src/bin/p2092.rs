struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let n = 6;
    let meetings = vec![
      vec![1, 2, 5],
      vec![2, 3, 8],
      vec![1, 5, 10],
    ];
    let expected = vec![0, 1, 2, 3, 5];
    let first_person = 1;
    let actual = Solution::find_all_people(n, meetings, first_person);

    use std::collections::HashSet;
    let expected: HashSet<i32> = expected.into_iter().collect();
    let actual: HashSet<i32> = actual.into_iter().collect();
    assert_eq!(expected, actual);
  }

  #[test]
  fn test25() {
    let n = 6;
    let meetings = vec![
      vec![3, 1, 3],
      vec![1, 2, 2],
      vec![0, 3, 3],
    ];
    let expected = vec![0, 1, 3];
    let first_person = 3;
    let actual = Solution::find_all_people(n, meetings, first_person);

    use std::collections::HashSet;
    let expected: HashSet<i32> = expected.into_iter().collect();
    let actual: HashSet<i32> = actual.into_iter().collect();
    assert_eq!(expected, actual);
  }

  #[test]
  fn test26() {
    let n = 6;
    let meetings = vec![
      vec![0, 2, 1],
      vec![1, 3, 1],
      vec![4, 5, 1],
    ];
    let expected = vec![0, 1, 2, 3];
    let first_person = 1;
    let actual = Solution::find_all_people(n, meetings, first_person);

    use std::collections::HashSet;
    let expected: HashSet<i32> = expected.into_iter().collect();
    let actual: HashSet<i32> = actual.into_iter().collect();
    assert_eq!(expected, actual);
  }
}

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
      let mut combined_meetings: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::new();
      for meeting in &meetings {
        let person_a = meeting[0];
        let person_b = meeting[1];
        let time = meeting[2];

        let combined_meeting = combined_meetings.entry(time).or_default();
        combined_meeting.entry(person_a).or_default().push(person_b);
        combined_meeting.entry(person_b).or_default().push(person_a);
      }

      let mut sorted_meeting_times = combined_meetings.keys().collect::<Vec<_>>();
      sorted_meeting_times.sort();
      let mut knows_secret = vec![false; n as usize];
      knows_secret[first_person as usize] = true;
      knows_secret[0] = true;

      for time in &sorted_meeting_times {
        let meetings_at_time = combined_meetings.get(time).unwrap();
        let mut to_add = HashSet::new();
        for person in meetings_at_time.keys() {
          if knows_secret[*person as usize] {
            let adjacent_people = meetings_at_time.get(person).unwrap();
            for adjacent_person in adjacent_people {
              if !knows_secret[*adjacent_person as usize] {
                to_add.insert(*adjacent_person);
              }
            }
          }
        }

        let mut to_add: Vec<_> = to_add.into_iter().collect();
        while !to_add.is_empty() {
          let last = to_add.pop().unwrap();
          knows_secret[last as usize] = true;
          if let Some(adjacent_people) = meetings_at_time.get(&last) {
            for adjacent_person in adjacent_people {
              if !knows_secret[*adjacent_person as usize] {
                knows_secret[*adjacent_person as usize] = true;
                to_add.push(*adjacent_person);
              }
            }
          }
        }
      }

      let mut people_with_secret = vec![];
      for person in 0..n {
        if knows_secret[person as usize] {
          people_with_secret.push(person);
        }
      }

      return people_with_secret.into_iter().collect();
    }
}
