package leetcode

func ZigzagLevelOrder(root *TreeNode) [][]int {
    if root == nil {
        return nil
    }
    queue := []*TreeNode{root}
    var acc [][]int
    lvl := 0
    for true {
        t := []int{}
        last := len(queue)
        if last == 0 {
            return acc
        }
        for k, n := range(queue[0:last]) {
            if lvl & 1 == 1 {
                t = append(t, queue[last - k - 1].Val)
            } else {
                t = append(t, n.Val)
            }
            if n.Left != nil {
                queue = append(queue, n.Left)
            }
            if n.Right != nil {
                queue = append(queue, n.Right)
            }
        }
        acc = append(acc, t)
        queue = queue[last:]
        lvl += 1
    }
    return acc
}