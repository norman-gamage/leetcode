# 2807. Insert Greatest Common Divisors in Linked List
# https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
  def insertGreatestCommonDivisors(self, head: Optional[ListNode]) -> Optional[ListNode]:
    # Get Greatest Common Denominator between two integers
    # @param a Integer 1
    # @param b Integer 2
    # @returns Greatest Common Denominator
    def getGCD(a: int, b: int) -> int:
      return a if (b == 0) else getGCD(b, a % b)

    ###

    ptr = head

    while ptr.next != None:
      newNode = ListNode(getGCD(ptr.val, ptr.next.val), ptr.next)
      ptr.next = newNode
      ptr = newNode.next

    return head
