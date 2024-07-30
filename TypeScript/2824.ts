/**
 * 2824. Count Pairs Whose Sum is Less than Target
 * https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/
 */

function countPairs(nums: number[], target: number): number {
  let out: number = 0;

  for (let i: number = 0; i < nums.length - 1; i++) {
    for (let j: number = i + 1; j < nums.length; j++) {
      if (nums[i] + nums[j] < target) out += 1;
    }
  }

  return out;
}
