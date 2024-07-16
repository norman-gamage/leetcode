/**
 * 3190. Find Minimum Operations to Make All Elements Divisible by Three
 * https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/
 */

function minimumOperations(nums: number[]): number {
  let out: number = 0;

  nums.forEach((n: number) => {
    out += Math.min(n % 3, 3 - (n % 3));
  });

  return out;
}
