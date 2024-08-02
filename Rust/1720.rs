/**
 * 1720. Decode XORed Array
 * https://leetcode.com/problems/decode-xored-array/
 */

impl Solution {
  pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut out: Vec<i32> = vec![first];

    for i in 0..encoded.len() {
      out.push(encoded[i] ^ out[i]);
    }

    return out;
  }
}
