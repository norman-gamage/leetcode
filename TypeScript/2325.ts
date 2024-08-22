/**
 * 2325. Decode the Message
 * https://leetcode.com/problems/decode-the-message/
 */

function decodeMessage(key: string, message: string): string {
  const map = new Map<string, string>();
  const flag: boolean[] = Array(26).fill(false);
  let flag_idx: number = 0;
  let out: string = "";

  key.split("").forEach((k: string) => {
    let k_ascii: number = k.charCodeAt(0) - 97;

    if (k !== " " && !flag[k_ascii]) {
      map.set(k, String.fromCharCode(flag_idx + 97));
      flag[k_ascii] = true;
      flag_idx += 1;
    }
  });

  message.split("").forEach((m: string) => {
    out += m === " " ? " " : map.get(m);
  });

  return out;
}
