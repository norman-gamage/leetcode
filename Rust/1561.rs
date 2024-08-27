/**
 * 1561. Maximum Number of Coins You Can Get
 * https://leetcode.com/problems/maximum-number-of-coins-you-can-get/
 */

impl Solution {
  pub fn max_coins(piles: Vec<i32>) -> i32 {
    let mut piles: Vec<i32> = piles;
    piles.sort_unstable();
    let mut idx: usize = piles.len() / 3;
    let mut out: i32 = 0;

    while idx < piles.len() {
      out += piles[idx];
      idx += 2;
    }

    return out;
  }
}
