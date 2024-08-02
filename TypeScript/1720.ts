/**
 * 1720. Decode XORed Array
 * https://leetcode.com/problems/decode-xored-array/
 */

function decode(encoded: number[], first: number): number[] {
  const out: number[] = [first];

  for (let i: number = 0; i < encoded.length; i++) {
    out.push(encoded[i] ^ out[i]);
  }

  return out;
}
