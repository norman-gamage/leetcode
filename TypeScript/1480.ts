/**
 * 1480. Running Sum of 1d Array
 * https://leetcode.com/problems/running-sum-of-1d-array/
 */

function runningSum(nums: number[]): number[] {
  for (let i: number = 1; i < nums.length; i++) {
    nums[i] += nums[i - 1];
  }

  return nums;
}
