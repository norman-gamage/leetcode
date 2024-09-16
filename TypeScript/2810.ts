/**
 * 2810. Faulty Keyboard
 * https://leetcode.com/problems/faulty-keyboard/
 */

function finalString(s: string): string {
  let out: string = "";

  s.split("").forEach((c: string) => {
    if (c !== "i") {
      out += c;
    } else {
      out = out.split("").reverse().join("");
    }
  });

  return out;
}
