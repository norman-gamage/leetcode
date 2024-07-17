/**
 * 1512. Number of Good Pairs
 * https://leetcode.com/problems/number-of-good-pairs/
 */

function numIdenticalPairs(nums: number[]): number {
  const numCount = new Map<number, number>();
  let out: number = 0;

  // Count how many times each integer appears
  nums.forEach((n: number) => {
    numCount.has(n) ? numCount.set(n, numCount.get(n) + 1) : numCount.set(n, 1);
  });

  // Calculate number of good pairs
  numCount.forEach((v: number) => {
    out += v > 1 ? (v * (v - 1)) / 2 : 0;
  });

  return out;
}
