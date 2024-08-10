/**
 * 2974. Minimum Number Game
 * https://leetcode.com/problems/minimum-number-game/
 */

impl Solution {
  pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
    let mut nums: Vec<i32> = nums;
    nums.sort_unstable();

    for i in (0..nums.len()).step_by(2) {
      [nums[i], nums[i + 1]] = [nums[i + 1], nums[i]];
    }

    return nums;
  }
}
