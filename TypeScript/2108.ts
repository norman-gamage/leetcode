/**
 * 2108. Find First Palindromic String in the Array
 * https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
 */

function firstPalindrome(words: string[]): string {
  let out: string = "";

  for (let i: number = 0; i < words.length; i++) {
    let w: string = words[i];
    let _w: string = w.split("").reverse().join("");

    if (w === _w) {
      out = w;
      break;
    }
  }

  return out;
}
