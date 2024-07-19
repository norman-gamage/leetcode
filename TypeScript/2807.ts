/**
 * 2807. Insert Greatest Common Divisors in Linked List
 * https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/
 */

/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function insertGreatestCommonDivisors(head: ListNode | null): ListNode | null {
  /**
   * Get Greatest Common Denominator between two integers
   * @param a Integer 1
   * @param b Integer 2
   * @returns Greatest Common Denominator
   */
  const getGCD = (a: number, b: number): number => {
    return b === 0 ? a : getGCD(b, a % b);
  };

  let ptr: ListNode = head;

  while (ptr.next) {
    const newNode: ListNode = new ListNode(
      getGCD(ptr.val, ptr.next.val),
      ptr.next
    );

    ptr.next = newNode;
    ptr = newNode.next;
  }

  return head;
}
