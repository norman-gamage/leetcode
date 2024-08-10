/**
 * 2974. Minimum Number Game
 * https://leetcode.com/problems/minimum-number-game/
 */

function numberGame(nums: number[]): number[] {
  nums = nums.sort((a: number, b: number) => a - b);

  for (let i: number = 0; i < nums.length; i += 2) {
    [nums[i], nums[i + 1]] = [nums[i + 1], nums[i]];
  }

  return nums;
}
