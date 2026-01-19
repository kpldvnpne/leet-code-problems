pub struct Solution;

fn main() {

}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let ans = Solution::most_booked(2, vec![
      vec![0, 10],
      vec![1, 5],
      vec![2, 7],
      vec![3, 4],
    ]);

    assert_eq!(ans, 0);
  }

  #[test]
  fn test2() {
    let ans = Solution::most_booked(3, vec![
      vec![1,20],
      vec![2,10],
      vec![3,5],
      vec![4,9],
      vec![6,8]
    ]);

    assert_eq!(ans, 1);
  }

  #[test]
  fn test3() {
    let ans = Solution::most_booked(4, vec![
      vec![18, 19],
      vec![3, 12],
      vec![17, 19],
      vec![2, 13],
      vec![7, 10],
    ]);

    assert_eq!(ans, 0);
  }
}

use std::cmp::Ordering;

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Copy,Clone,Eq,PartialEq)]
struct MeetingEnd {
  end_time: i32,
  meeting_room: i32,
}

impl Ord for MeetingEnd {
  fn cmp(&self, other: &Self) -> Ordering {
    other.end_time.cmp(&self.end_time).then(other.meeting_room.cmp(&self.meeting_room))
  }
}

impl PartialOrd for MeetingEnd {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
      let mut booked_count: HashMap<i32, u32> = HashMap::with_capacity(n as usize);
      let mut sorted_meetings = meetings.clone();
      sorted_meetings.sort_by(|a, b| { a[0].cmp(&b[0]) });

      // println!("{}", sorted_meetings.iter().map(|it| format!("({}, {})", it[0], it[1])).collect::<Vec<String>>().join(", "));

      let mut free_rooms: BinaryHeap<_> = (0..n).into_iter().map(Reverse).collect();
      let mut meeting_ends: BinaryHeap<MeetingEnd> = BinaryHeap::new();

      for meeting in sorted_meetings {
        let mut start_time = meeting[0];
        let mut end_time = meeting[1];

        // Remove every free rooms
        loop {
          if let Some(meeting_end) = meeting_ends.peek() {
            if meeting_end.end_time > start_time {
              if !free_rooms.is_empty() {
                break;
              } else {
                let interval = end_time - start_time;
                start_time = meeting_end.end_time;
                end_time = start_time + interval;

                free_rooms.push(Reverse(meeting_end.meeting_room));
                meeting_ends.pop();
                break;
              }
            }

            free_rooms.push(Reverse(meeting_end.meeting_room));
            meeting_ends.pop();
          } else {
            break;
          }
        }

        if let Some(Reverse(free_room)) = free_rooms.pop() {
          let count = booked_count.entry(free_room).or_default();
          *count += 1;
          meeting_ends.push(MeetingEnd {
            end_time,
            meeting_room: free_room,
          });
        }
      }

      let mut max_count: u32 = 0;
      let mut max_count_room = 0;
      for room in 0..n {
        let count = booked_count.entry(room).or_default();
        if *count > max_count {
          max_count = *count;
          max_count_room = room;
        }
      }

      return max_count_room;
    }
}
