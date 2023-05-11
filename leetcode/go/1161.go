package leetcode

/*
    1161. Maximum Level Sum of a Binary Tree
    ----------------------------------------
*/

func maxLevelSum(root *TreeNode) int {

    maxVal, maxLvl := 0, 0
    lvl := 0

    var bfs func(q []*TreeNode)
    bfs = func(q []*TreeNode) {
        if len(q) == 0 {
            return
        }
        l := len(q)
        sum := 0
        lvl += 1
        for k := range q {
            sum += q[k].Val
            if q[k].Left != nil {
                q = append(q, q[k].Left)
            }
            if q[k].Right != nil {
                q = append(q, q[k].Right)
            }
        }
        if lvl == 1 || sum > maxVal {
            maxVal = sum
            maxLvl = lvl
        }
        bfs(q[l:])
    }

    bfs([]*TreeNode{root})
    return maxLvl
}
