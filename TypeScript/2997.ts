/**
 * 2997. Minimum Number of Operations to Make Array XOR Equal to K
 * https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/
 */

function minOperations(nums: number[], k: number): number {
  return (k ^ nums.reduce((a: number, b: number) => a ^ b, 0))
    .toString(2)
    .split("")
    .map(Number)
    .reduce((a: number, b: number) => a + b, 0);
}
