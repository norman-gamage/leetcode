/**
 * 2181. Merge Nodes in Between Zeros
 * https://leetcode.com/problems/merge-nodes-in-between-zeros/
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

function mergeNodes(head: ListNode | null): ListNode | null {
  let back: ListNode = head;
  let front: ListNode = head.next;

  while (front) {
    if (!front.next) {
      back.next = null;
      break;
    } else if (front.val === 0) {
      back.next = front;
      back = back.next;
    } else {
      back.val += front.val;
    }

    front = front.next;
  }

  return head;
}
