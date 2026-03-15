fn main() {}

enum OpType {
  Add,
  Mult
}

struct Op {
  operand: i64,
  op_type: OpType,
}

struct Fancy {
  list: Vec<i64>,
  ops: Vec<Op>,
  last_ops: Vec<usize>
}


const MOD: i64 = 1_000_000_000 + 7;

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
        ops: vec![],
        last_ops: vec![]
      }
    }

    fn append(&mut self, val: i32) {
      self.list.push(val as i64);
      self.last_ops.push(self.ops.len());
    }

    fn add_all(&mut self, inc: i32) {
      self.ops.push(Op{operand: inc as i64, op_type: OpType::Add});
    }

    fn mult_all(&mut self, m: i32) {
      self.ops.push(Op{operand: m as i64, op_type: OpType::Mult});
    }

    fn get_index(&mut self, idx: i32) -> i32 {
      let idx = idx as usize;
      if idx >= self.list.len() {
        return -1;
      }
      let mut result = self.list[idx];
      for op in &self.ops[self.last_ops[idx]..self.ops.len()] {
        if matches!(op.op_type, OpType::Add) {
          result = (result + op.operand) % MOD;
        } else {
          result = (result * op.operand) % MOD;
        }
      }
      self.list[idx] = result;
      self.last_ops[idx] = self.ops.len();
      return result as i32;
    }
}
