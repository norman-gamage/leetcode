/**
 * 2006. Count Number of Pairs With Absolute Difference K
 * https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k/
 */

function countKDifference(nums: number[], k: number): number {
  let len: number = nums.length;
  let out: number = 0;

  for (let i: number = 0; i < len - 1; i++) {
    for (let j: number = i + 1; j < len; j++) {
      if (Math.abs(nums[i] - nums[j]) === k) {
        out += 1;
      }
    }
  }

  return out;
}
