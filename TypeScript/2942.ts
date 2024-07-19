/**
 * 2942. Find Words Containing Character
 * https://leetcode.com/problems/find-words-containing-character/
 */

function findWordsContaining(words: string[], x: string): number[] {
  const out: number[] = [];

  words.forEach((v, i) => {
    if (v.includes(x)) out.push(i);
  });

  return out;
}
