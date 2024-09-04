/**
 * 1656. Design an Ordered Stream
 * https://leetcode.com/problems/design-an-ordered-stream/
 */

struct OrderedStream {
  arr: Vec<String>,
  idx: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
  fn new(n: i32) -> Self {
    OrderedStream {
      arr: vec!["".to_string(); n as usize],
      idx: 0,
    }
  }

  fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
    self.arr[(id_key - 1) as usize] = value;

    let mut out: Vec<String> = Vec::new();

    while self.idx < (self.arr.len() as i32) && self.arr[self.idx as usize] != "" {
      out.push(self.arr[self.idx as usize].clone());
      self.idx += 1;
    }

    return out;
  }
}
