func DelNodes(root *TreeNode, to_delete []int) []*TreeNode {
    td := make([]int, 1001)
    forest := []*TreeNode{}

    for i := range to_delete {
        td[to_delete[i]] = 1
    }

    var dfs func(node *TreeNode, is_root bool)

    dfs = func(node *TreeNode, is_root bool) {
        if node == nil {
            return
        }
        if td[node.Val] == 1 {
            dfs(node.Left, true)
            dfs(node.Right, true)
        } else {
            if node.Left != nil {
                if td[node.Left.Val] == 1 {
                    dfs(node.Left, true)
                    node.Left = nil
                } else {
                    dfs(node.Left, false)
                }
            }
            if node.Right != nil {
                if td[node.Right.Val] == 1 {
                    dfs(node.Right, true)
                    node.Right = nil
                } else {
                    dfs(node.Right, false)
                }
            }
            if is_root {
                forest = append(forest, node)
            }
        }
    }
    dfs(root, true)
    return forest
}
