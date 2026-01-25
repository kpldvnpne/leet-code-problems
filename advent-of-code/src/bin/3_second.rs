fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

fn get_number(digits: &Vec<u8>) -> u64 {
  let mut number: u64 = 0;
  for digit in digits {
    number = number * 10 + *digit as u64;
  }
  number
}

fn main() {
  let mut ans: u64 = 0;
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    // println!("{}", line);


    let line_digits: Vec<u8> = line.as_bytes().into_iter().map(|it| it - '0' as u8).collect();
    let mut max_digits: Vec<u8> = line_digits[..12].iter().map(|it| *it).collect();
    for i in 12..line_digits.len() {
      let mut new_max_digits = max_digits.clone();
      for skip_i in 0..12 {
        let mut new_digits = Vec::with_capacity(12);
        for (i, digit) in max_digits.iter().enumerate() {
          if i != skip_i {
            new_digits.push(*digit);
          }
        }
        new_digits.push(line_digits[i]);

        if get_number(&new_digits) > get_number(&new_max_digits) {
          new_max_digits = new_digits;
        }
      }

      max_digits = new_max_digits;
    }

    let max_battery_capacity = get_number(&max_digits);

    ans += max_battery_capacity as u64;
  }

  println!("{}", ans);
}
