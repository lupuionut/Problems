package leetcode

/*
    100. Same Tree
    --------------
*/

func isSameTree(p *TreeNode, q *TreeNode) bool {
    if (p == nil && q != nil) || (q == nil && p != nil) {
        return false
    }
    if p.Val != q.Val {
        return false
    }
    if p == nil && q == nil {
        return true
    }
    return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}
