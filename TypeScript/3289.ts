/**
 * 3289. The Two Sneaky Numbers of Digitville
 * https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/
 */

function getSneakyNumbers(nums: number[]): number[] {
  const cnt: number[] = Array(nums.length).fill(0);
  const out: number[] = [];

  for (let i: number = 0; i < nums.length; i++) {
    const n: number = nums[i];
    cnt[n] += 1;

    if (cnt[n] === 2) {
      out.push(n);
    }
  }

  return out;
}
