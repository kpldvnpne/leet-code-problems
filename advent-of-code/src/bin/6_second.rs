fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

fn main() {
  let mut table = vec![];
  let mut max_len = 0;
  loop {
    let mut line = read_line();
    // println!("{}", line);

    // if line.is_empty() {
    //   break;
    // }

    line.remove(line.len() - 1);

    if line.is_empty() {
      break;
    }

    max_len = max_len.max(line.len());

    table.push(line.to_string());
  }

  let operators = table.pop().unwrap().split_ascii_whitespace().map(|it| it.trim().to_string()).collect::<Vec<_>>();

  let table = table.into_iter().map(|it| {
    // if it.len() < max_len {
    //   for _ in 0..(max_len - it.len()) {
    //     it.push(' ');
    //   }
    // }
    it.as_bytes().to_owned()
  }).collect::<Vec<_>>();

  let mut total_result: u128 = 0;
  let mut j: i32 = table[0].len() as i32 - 1;
  for operator in operators.into_iter().rev() {
    let mut col_result = if operator == "*" { 1 } else { 0 };

    while j >= 0 {
      let mut number: u128 = 0;
      let mut found_digit = false;
      for i in 0..table.len() {
        // println!("{number}");
        let j = j as usize;
        if '0' as u8 <= table[i][j] && table[i][j] <= '9' as u8 {
          number = number * 10 + (table[i][j] - '0' as u8) as u128;
          found_digit = true;
        }
      }

      j = j - 1;

      if !found_digit {
        break;
      }

      // println!("Number: {}", number);

      col_result = if operator == "*" { col_result * number } else { col_result + number };
    }

    // println!("Col result = {}", col_result);

    total_result += col_result;
  }
  println!("{}", total_result);
}
