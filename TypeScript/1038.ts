/**
 * 1038. Binary Search Tree to Greater Sum Tree
 * https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
 */

/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function bstToGst(root: TreeNode | null): TreeNode | null {
  const reverseInorderTraversal = (
    node: TreeNode | null,
    sum: number = 0
  ): number => {
    if (node.right) {
      sum = reverseInorderTraversal(node.right, sum);
    }

    sum += node.val;
    node.val = sum;

    if (node.left) {
      sum = reverseInorderTraversal(node.left, sum);
    }

    return sum;
  };

  reverseInorderTraversal(root);

  return root;
}
