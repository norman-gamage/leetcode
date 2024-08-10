/**
 * 2652. Sum Multiples
 * https://leetcode.com/problems/sum-multiples/
 */

impl Solution {
  pub fn sum_of_multiples(n: i32) -> i32 {
    let mut out: i32 = 0;

    for i in 0..=n {
      if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
        out += i;
      }
    }

    return out;
  }
}
