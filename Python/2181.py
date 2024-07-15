# 2181. Merge Nodes in Between Zeros
# https://leetcode.com/problems/merge-nodes-in-between-zeros/

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
  def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
    back = head
    front = head.next

    while front is not None:
      if front.next is None:
        back.next = None
        break
      elif front.val == 0:
        back.next = front
        back = back.next
      else:
        back.val += front.val

      front = front.next

    return head
