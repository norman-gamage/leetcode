/**
 * 2574. Left and Right Sum Differences
 * https://leetcode.com/problems/left-and-right-sum-differences/submissions/1339247246/
 */

function leftRightDifference(nums: number[]): number[] {
  const sum: number = nums.reduce((a: number, b: number) => a + b, 0);
  const out: number[] = Array(nums.length).fill(0);
  let leftSum: number = 0;

  for (let i: number = 0; i < nums.length; i++) {
    out[i] = Math.abs(leftSum - (sum - nums[i] - leftSum));
    leftSum += nums[i];
  }

  return out;
}
