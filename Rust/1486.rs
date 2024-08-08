/**
 * 1486. XOR Operation in an Array
 * https://leetcode.com/problems/xor-operation-in-an-array/
 */

impl Solution {
  pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut start: i32 = start;
    let mut out: i32 = 0;

    for i in 0..n {
      out ^= start;
      start += 2;
    }

    return out;
  }
}
