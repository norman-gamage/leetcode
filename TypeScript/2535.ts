/**
 * 2535. Difference Between Element Sum and Digit Sum of an Array
 * https://leetcode.com/problems/difference-between-element-sum-and-digit-sum-of-an-array/
 */

function differenceOfSum(nums: number[]): number {
  let element_sum: number = 0;
  let digit_sum: number = 0;

  nums.forEach((n: number) => {
    element_sum += n;

    while (n > 0) {
      digit_sum += n % 10;
      n = Math.floor(n / 10);
    }
  });

  return element_sum - digit_sum;
}
