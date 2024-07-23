/**
 * 1863. Sum of All Subset XOR Totals
 * https://leetcode.com/problems/sum-of-all-subset-xor-totals/
 */

function subsetXORSum(nums: number[]): number {
  return nums.reduce((a, b) => a | b) << (nums.length - 1);
}
