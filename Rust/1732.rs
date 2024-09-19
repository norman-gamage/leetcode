/**
 * 1732. Find the Highest Altitude
 * https://leetcode.com/problems/find-the-highest-altitude/
 */

impl Solution {
  pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut height: i32 = 0;
    let mut out: i32 = 0;

    for g in gain {
      height += g;

      if height > out {
        out = height;
      }
    }

    return out;
  }
}
