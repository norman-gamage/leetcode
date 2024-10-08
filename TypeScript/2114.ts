/**
 * 2114. Maximum Number of Words Found in Sentences
 * https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
 */

function mostWordsFound(sentences: string[]): number {
  return Math.max(...sentences.map((s: string) => s.split(" ").length));
}
