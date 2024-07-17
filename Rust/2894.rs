/**
 * 2894. Divisible and Non-divisible Sums Difference
 * https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/
 */

impl Solution {
  pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut out: i32 = 0;

    for i in 1..n + 1 {
      out += if i % m != 0 { i } else { -i };
    }

    return out;
  }
}
