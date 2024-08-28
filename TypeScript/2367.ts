/**
 * 2367. Number of Arithmetic Triplets
 * https://leetcode.com/problems/number-of-arithmetic-triplets/
 */

function arithmeticTriplets(nums: number[], diff: number): number {
  let len: number = nums.length;
  let out: number = 0;

  for (let i: number = 0; i < len - 2; i++) {
    for (let j: number = i + 1; j < len - 1; j++) {
      if (nums[j] - nums[i] < diff) {
        continue;
      } else if (nums[j] - nums[i] > diff) {
        break;
      } else {
        for (let k: number = j + 1; k < len; k++) {
          if (nums[k] - nums[j] < diff) {
            continue;
          } else if (nums[k] - nums[j] > diff) {
            break;
          } else {
            out += 1;
            break;
          }
        }
      }
    }
  }

  return out;
}
