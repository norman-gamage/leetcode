# 1720. Decode XORed Array
# https://leetcode.com/problems/decode-xored-array/

class Solution:
  def decode(self, encoded: List[int], first: int) -> List[int]:
    out = [first]

    for i in range(0, len(encoded)):
      out.append(encoded[i] ^ out[i])

    return out
