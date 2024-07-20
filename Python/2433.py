# 2433. Find The Original Array of Prefix Xor
# https://leetcode.com/problems/find-the-original-array-of-prefix-xor/submissions/1327260397/

class Solution:
  def findArray(self, pref: List[int]) -> List[int]:
    out = [pref[0]]

    for i in range(1, len(pref)):
      out.append(pref[i] ^ pref[i - 1])

    return out
