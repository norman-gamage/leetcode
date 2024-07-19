/**
 * 771. Jewels and Stones
 * https://leetcode.com/problems/jewels-and-stones/
 */

impl Solution {
  pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut out = 0;

    for s in stones.chars() {
      if jewels.find(s) != None {
        out += 1;
      }
    }

    return out;
  }
}
