/**
 * 2160. Minimum Sum of Four Digit Number After Splitting Digits
 * https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
 */

function minimumSum(num: number): number {
  const arr: number[] = [];

  while (num !== 0) {
    arr.push(num % 10);
    num = Math.floor(num/10);
  }

  for (let i: number = 0; i < 2; i++) {
    for (let j: number = 2; j < 4; j++) {
      if (arr[i] > arr[j]) [arr[i], arr[j]] = [arr[j], arr[i]];
    }
  }

  return arr[0] * 10 + arr[2] + arr[1] * 10 + arr[3];
}
