package leetcode

/*
    1372. Longest ZigZag Path in a Binary Tree
    ------------------------------------------
    Try to visit starting from root going to left - right - left
    and right - left - right ... until we find a nil node.
    For this traversal, keep a counter(level) and check it
    against current max value.
    For each visited node, start a new traversal for the opposite
    direction, for ex if we visit left node, we should continue with
    right child node and start another traversal for left child node.
*/

func longestZigZag(root *TreeNode) int {

    var visitLeft func(*TreeNode, int)
    var visitRight func(*TreeNode, int)
    max := 0

    visitLeft = func(node *TreeNode, level int) {
        if node == nil {
            if level > max {
                max = level
            }
            return
        }
        visitLeft(node.Left, 0)
        visitRight(node.Right, level + 1)
    }

    visitRight = func(node *TreeNode, level int) {
        if node == nil {
            if level > max {
                max = level
            }
            return
        }
        visitRight(node.Right, 0)
        visitLeft(node.Left, level + 1)
    }

    visitLeft(root.Left, 0)
    visitRight(root.Right, 0)

    return max
}
