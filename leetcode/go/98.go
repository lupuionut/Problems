package leetcode

/*
    98. Validate Binary Search Tree
    -------------------------------
*/

func isValidBST(root *TreeNode) bool {
    const MINIMUM = (-1 << 31) - 1
    const MAXIMUM =  (1 << 31)

    var validNode func(node *TreeNode, min, max int) bool
    validNode = func(n *TreeNode, min, max int) bool {
        if n == nil {
            return true
        }
        if n.Val >= max || n.Val <= min {
            return false
        }
        return validNode(n.Left, min, n.Val) && validNode(n.Right, n.Val, max)
    }

    return validNode(root.Left, MINIMUM, root.Val) && validNode(root.Right, root.Val, MAXIMUM)
}
