package leetcode

func BuildTree(inorder []int, postorder []int) *TreeNode {
    hm := make(map[int]int)
    l := len(inorder) - 1

    for k := range inorder {
        hm[inorder[k]] = k
    }

    return buildTreeRec(inorder, 0, l, postorder, 0, l, hm)
}

func buildTreeRec(inorder []int, is, ie int, postorder []int, ps, pe int, hm map[int]int) *TreeNode {
    if is > ie || ps > pe {
        return nil
    }

    node := &TreeNode{Val: postorder[pe]}
    idx := hm[postorder[pe]]
    delta := idx - is
    node.Left = buildTreeRec(inorder, is, idx-1, postorder, ps, ps+delta-1, hm);
    node.Right = buildTreeRec(inorder, idx+1, ie, postorder, ps+delta, pe-1, hm);

    return node
}
