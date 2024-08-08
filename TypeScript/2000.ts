/**
 * 2000. Reverse Prefix of Word
 * https://leetcode.com/problems/reverse-prefix-of-word/
 */

function reversePrefix(word: string, ch: string): string {
  const firstInd: number = word.indexOf(ch) + 1;

  return firstInd < 0
    ? word
    : word
        .slice(0, firstInd)
        .split("")
        .reverse()
        .join("")
        .concat(word.slice(firstInd));
}
