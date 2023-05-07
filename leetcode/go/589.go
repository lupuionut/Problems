package leetcode

/*
    589. N-ary Tree Preorder Traversal
    ----------------------------------
*/

func preorder(root *Node) []int {
    res := new([]int)

    var po func(n *Node, acc *[]int)
    po = func(n *Node, acc *[]int) {
        if n == nil {
            return
        }
        *acc = append(*acc, n.Val)
        if len(n.Children) > 0 {
            for k := range n.Children {
                po(n.Children[k], acc)
            }
        }
    }
    po(root, res)
    return *res
}
