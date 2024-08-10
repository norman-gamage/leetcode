/**
 * 1389. Create Target Array in the Given Order
 * https://leetcode.com/problems/create-target-array-in-the-given-order/
 */

function createTargetArray(nums: number[], index: number[]): number[] {
  const out: number[] = [];

  for (let i: number = 0; i < nums.length; i++) {
    out.splice(index[i], 0, nums[i]);
  }

  return out;
}
