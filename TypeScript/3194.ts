/**
 * 3194. Minimum Average of Smallest and Largest Elements
 * https://leetcode.com/problems/minimum-average-of-smallest-and-largest-elements/
 */

function minimumAverage(nums: number[]): number {
  nums = nums.sort((a: number, b: number) => a - b);
  let out: number = Number.MAX_VALUE;

  for (let i: number = 0; i < nums.length / 2; i++) {
    const avg: number = (nums[i] + nums[nums.length - i - 1]) / 2;

    if (avg < out) {
      out = avg;
    }
  }

  return out;
}
