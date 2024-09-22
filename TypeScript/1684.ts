/**
 * 1684. Count the Number of Consistent Strings
 * https://leetcode.com/problems/count-the-number-of-consistent-strings/
 */

function countConsistentStrings(allowed: string, words: string[]): number {
  let out: number = 0;

  words.forEach((word: string) => {
    let flag: boolean = false;

    for (let i: number = 0; i < word.length; i++) {
      flag = allowed.indexOf(word[i]) < 0;
      if (flag) break;
    }

    if (!flag) out += 1;
  });

  return out;
}
