/**
 * 1832. Check if the Sentence Is Pangram
 * https://leetcode.com/problems/check-if-the-sentence-is-pangram/
 */

function checkIfPangram(sentence: string): boolean {
  if (sentence.length < 26) {
    return false;
  }

  const alphabet: string = "abcdefghijklmnopqrstuvwxyz";

  for (let i: number = 0; i < alphabet.length; i++) {
    if (sentence.indexOf(alphabet[i]) < 0) {
      return false;
    }
  }

  return true;
}
