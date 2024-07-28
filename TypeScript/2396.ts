/**
 * 2396. Strictly Palindromic Number
 * https://leetcode.com/problems/strictly-palindromic-number/
 */

function isStrictlyPalindromic(n: number): boolean {
  for (let i: number = 2; i <= n - 2; i++) {
    const str: string = n.toString(i);

    for (let j: number = 0; j < str.length / 2; j++) {
      if (str[j] !== str[str.length - 1 - j]) return false;
    }
  }

  return true;
}
