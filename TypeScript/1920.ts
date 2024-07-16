/**
 * 1920. Build Array from Permutation
 * https://leetcode.com/problems/build-array-from-permutation/
 */

function buildArray(nums: number[]): number[] {
  const out: number[] = Array(nums.length);

  for (let i: number = 0; i < nums.length; i++) {
    out[i] = nums[nums[i]];
  }

  return out;
}
