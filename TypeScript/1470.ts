/**
 * 1470. Shuffle the Array
 * https://leetcode.com/problems/shuffle-the-array/
 */

function shuffle(nums: number[], n: number): number[] {
  const out: number[] = Array(2 * n);

  for (let i: number = 0; i < n; i++) {
    out[2 * i] = nums[i];
    out[2 * i + 1] = nums[i + n];
  }

  return out;
}
