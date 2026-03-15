fn main() {}

struct Fancy {
  list: Vec<i64>,
  mult: i64,
  add: i64
}

const MOD: i64 = 1_000_000_000 + 7;

fn bit_pow(base: i64, pow: i64) -> i64 {
  let mut base = base;
  let mut result = 1;
  for i in 0..30 { // NOTE: Only works for pow < 10^9
    if (pow >> i) & 1 == 1 {
      result = (result * base) % MOD;
    }
    base = (base * base) % MOD;
  }
  return result;
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */
impl Fancy {

    fn new() -> Self {
      Fancy {
        list: vec![],
        mult: 1,
        add: 0,
      }
    }

    fn append(&mut self, val: i32) {
      let x = ((val as i64 - self.add + MOD) % MOD * bit_pow(self.mult, MOD - 2)) % MOD;
      self.list.push(x);
    }

    fn add_all(&mut self, inc: i32) {
      self.add = (self.add + inc as i64) % MOD;
    }

    fn mult_all(&mut self, m: i32) {
      let m = m as i64;
      self.mult = (self.mult * m) % MOD;
      self.add = (self.add * m) % MOD;
    }

    fn get_index(&mut self, idx: i32) -> i32 {
      let idx = idx as usize;
      if idx >= self.list.len() {
        return -1;
      }
      let x = self.list[idx];
      return (((x * self.mult) % MOD + self.add) % MOD) as i32;
    }
}
