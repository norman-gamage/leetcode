/**
 * 2325. Decode the Message
 * https://leetcode.com/problems/decode-the-message/
 */

use std::collections::HashMap;

impl Solution {
  pub fn decode_message(key: String, message: String) -> String {
    let mut map: HashMap<char, char> = HashMap::new();
    let mut flag: Vec<bool> = vec![false; 26];
    let mut flag_idx: u32 = 0;
    let mut out: String = "".to_string();

    for k in key.chars() {
      let k_ascii: usize = (k.to_ascii_lowercase() as usize) - 97;

      if !(k == ' ' || flag[k_ascii] == true) {
        map.insert(k, char::from_u32(flag_idx + 97).unwrap());
        flag[k_ascii] = true;
        flag_idx += 1;
      }
    }

    for m in message.chars() {
      out.push(if m == ' ' { ' ' } else { *map.get(&m).unwrap() });
    }

    return out;
  }
}
