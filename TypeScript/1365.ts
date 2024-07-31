/**
 * 1365. How Many Numbers Are Smaller Than the Current Number
 * https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
 */

function smallerNumbersThanCurrent(nums: number[]): number[] {
  let out: number[] = Array(nums.length);
  const sortedNums: number[] = [...nums].sort((a: number, b: number) => a - b);

  nums.forEach((v: number, i: number) => (out[i] = sortedNums.indexOf(v)));

  return out;
}
