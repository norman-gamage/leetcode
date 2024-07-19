
/**
 * 2807. Insert Greatest Common Divisors in Linked List
 * https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/
 */

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
  pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    /**
     * Get Greatest Common Denominator between two integers
     * @param a Integer 1
     * @param b Integer 2
     * @returns Greatest Common Denominator
     */
    fn getGCD(mut a: i32, mut b: i32) -> i32 {
      return if b == 0 { a } else { getGCD(b, a % b) };
    }

    // Make head mutable
    let mut head = head;

    // Pointer to traverse through
    let mut ptr = &mut head;

    while let Some(node) = ptr {
      // Exit if last node
      if node.next == None {
        break;
      }

      // Construct new node
      let mut new_node = ListNode {
        val: getGCD(node.val, node.next.as_ref().unwrap().val),
        next: node.as_mut().next.take(),
      };

      // Link new node
      node.next = Some(Box::new(new_node));

      // Update pointer
      ptr = &mut node.next.as_mut().unwrap().next;
    }

    return head;
  }
}
