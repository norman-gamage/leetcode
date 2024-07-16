/**
 * 2011. Final Value of Variable After Performing Operations
 * https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
 */

impl Solution {
  pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut out: i32 = 0;

    for o in operations {
      out += if o == "X++" || o == "++X" { 1 } else { -1 };
    }

    return out;
  }
}
