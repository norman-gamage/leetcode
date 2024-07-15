/**
 * 2181. Merge Nodes in Between Zeros
 * https://leetcode.com/problems/merge-nodes-in-between-zeros/
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
  pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut front = head.as_ref();
    let mut head = Box::new(ListNode::new(0));
    let mut back = &mut head;

    while let Some(node) = front {
      front = node.next.as_ref();

      if node.next.is_none() {
        break;
      } else if node.val == 0 {
        back.next = Some(Box::new(ListNode::new(0)));
        back = back.next.as_mut().unwrap();
      } else {
        back.val += node.val;
      }
    }

    return head.next;
  }
}
