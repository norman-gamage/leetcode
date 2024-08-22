# 2325. Decode the Message
# https://leetcode.com/problems/decode-the-message/

class Solution:
  def decodeMessage(self, key: str, message: str) -> str:
    map = {}
    flag = [False] * 26
    flag_idx = 0
    out = ""

    for k in key:
      k_ascii = ord(k) - 97

      if (k != " " and flag[k_ascii] == False):
        map[k] = chr(flag_idx + 97)
        flag[k_ascii] = True
        flag_idx += 1

    for m in message:
      if (m == " "):
        out += " "
      else:
        out += map.get(m)

    return out
