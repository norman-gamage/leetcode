/**
 * 2520. Count the Digits That Divide a Number
 * https://leetcode.com/problems/count-the-digits-that-divide-a-number/
 */

function countDigits(num: number): number {
  let _num: number = num;
  let out: number = 0;

  while (_num > 0) {
    let lastDigit: number = _num % 10;

    if (num % lastDigit === 0) {
      out += 1;
    }

    _num = Math.floor(_num / 10);
  }

  return out;
}
