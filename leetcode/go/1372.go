package leetcode

/*
    1372. Longest ZigZag Path in a Binary Tree
    ------------------------------------------
    Try to visit starting from root going to left - right - left
    and right - left - right ... until we find a nil node.
    For this traversal, keep a counter(edges) and check it
    against current max value.
    For each visited node, start a new traversal for the opposite
    direction, for ex if we visit left node, we should continue with
    right child node and start another traversal for left child node.
*/

func longestZigZag(root *TreeNode) int {

    var visitLeft func(*TreeNode, int)
    var visitRight func(*TreeNode, int)
    max := 0

    visitLeft = func(node *TreeNode, edges int) {
        if node == nil {
            if edges > max {
                max = edges
            }
            return
        }
        visitLeft(node.Left, 0)
        visitRight(node.Right, edges + 1)
    }

    visitRight = func(node *TreeNode, edges int) {
        if node == nil {
            if edges > max {
                max = edges
            }
            return
        }
        visitRight(node.Right, 0)
        visitLeft(node.Left, edges + 1)
    }

    visitLeft(root.Left, 0)
    visitRight(root.Right, 0)

    return max
}
