/**
 * 3065. Minimum Operations to Exceed Threshold Value I
 * https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-i/
 */

function minOperations(nums: number[], k: number): number {
  let out: number = 0;

  nums.forEach((v: number) => {
    if (v < k) out += 1;
  });

  return out;
}
