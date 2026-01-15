fn read_line() -> String {
  let mut str = String::new();
  std::io::stdin().read_line(&mut str).unwrap();
  return str;
}

fn read_int_line() -> i32 {
  read_line().trim().parse().unwrap()
}

fn read_int_vec_line() -> Vec<i32> {
  read_line().trim().split_ascii_whitespace().map(|it| it.parse().unwrap()).collect()
}

const MODULO: i32 = 998244353;

fn factorial(n: i32) -> i32 {
  let mut product = 1;
  for i in 2..=n {
    product *= i;
    if product >= MODULO {
      product %= MODULO;
    }
  }
  return product;
}

fn 

fn main() {
  let t = read_int_line();
  for _ in 0..t {
    let n = read_int_line() as usize;
    let a = read_int_vec_line();
    let n_factorial = factorial(n as i32);

    let mut answer = 0;
    for p0_i in 0..(n+1) {
      let mut max = i32::MIN;
      for p_i in 0..(n+1) {
        if p_i == p0_i {
          continue;
        }

        max = std::cmp::max(a[p_i], max);
      }

      let mut needed = 0;
      for p_i in 0..(n+1) {
        if p_i == p0_i {
          continue;
        }
        needed += max - a[p_i];
      }

      if needed <= a[p0_i] {
        answer += n_factorial;
      } else {
        let more = needed - a[p0_i];
        if more < n as i32 {
          answer += factorial(n as i32 - more) * factorial(more) * n as i32;
        }
      }
    }

    println!("{answer}");
  }
}
