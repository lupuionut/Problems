package leetcode

func MaxDepth(root *TreeNode) int {
    max := 0
    traverseBinTree(root, nil, &max)
    return max
}

func traverseBinTree(node *TreeNode, parent *TreeNode, max *int) {
    if parent == nil {
        if node == nil {
            return
        } else {
            node.Val = 1
        }
    } else {
        node.Val = parent.Val + 1
    }
    if node.Left != nil {
        traverseBinTree(node.Left, node, max)
    }
    if node.Right != nil {
        traverseBinTree(node.Right, node, max)
    }
    if node.Left == nil && node.Right == nil {
        if node.Val > *max {
            *max = node.Val
        }
    }
    return
}

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}
