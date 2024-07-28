/**
 * 1431. Kids With the Greatest Number of Candies
 * https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
 */

impl Solution {
  pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut out: Vec<bool> = Vec::new();
    let max: i32 = *candies.iter().max().unwrap();

    for v in candies {
      out.push(v + extra_candies >= max);
    }

    return out;
  }
}
