fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

fn count_rolls(rolls: &Vec<Vec<u8>>, i: i32, j: i32) -> u32 {
  let mut count = 0;
  for i_delta in &[-1, 0, 1] {
    let new_i = i + i_delta;
    if new_i < 0 || new_i as usize >= rolls.len() {
      continue;
    }
    for j_delta in &[-1, 0 ,1] {
      let new_j = j + j_delta;
      if new_j < 0 || new_j as usize >= rolls[0].len() || (i == new_i && j == new_j) {
        continue;
      }

      if rolls[new_i as usize][new_j as usize] == '@' as u8 {
        count += 1;
      }
    }
  }

  return count;
}

fn main() {
  let mut rolls = vec![];
  loop {
    let roll = read_line();
    let roll = roll.trim();
    if roll.is_empty() {
      break;
    } else {
      rolls.push(roll.as_bytes().to_owned());
    }
  }

  let mut ans = 0;
  for i in 0..rolls.len() {
    for j in 0..rolls[0].len() {
      if rolls[i][j] == '@' as u8 && count_rolls(&rolls, i as i32, j as i32) < 4 {
        // println!("{} {}", i, j);
        ans += 1;
      }
    }
  }

  println!("{}", ans);
}
