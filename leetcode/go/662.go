package leetcode

/*
    662. Maximum Width of Binary Tree
    ---------------------------------
    Run a bfs, each time we visit a level we modify the
    childrens node values to keep track of position for that
    level, so for ex the most left children has value 0.
    Left children node val = Parent val * 2
    Right children node val = Parent val * 2 + 1

                    0
            0             1
        0      1       2
    0     1         4

    In this case the last level has widh 5
    (end - start + 1 <=> 4 - 0 + 1)
*/

func widthOfBinaryTree(root *TreeNode) int {

    max := 0

    var bfs func([]*TreeNode)
    bfs = func(q []*TreeNode) {
        start := q[0].Val
        end := 0
        queue := []*TreeNode{}

        for k := range q {
            end = q[k].Val
            if q[k].Left != nil {
                q[k].Left.Val = q[k].Val * 2
                queue = append(queue, q[k].Left)
            }
            if q[k].Right != nil {
                q[k].Right.Val = q[k].Val * 2 + 1
                queue = append(queue, q[k].Right)
            }
        }

        current := end - start + 1
        if current > max {
            max = current
        }

        if len(queue) == 0 {
            return
        }
        bfs(queue)
    }

    root.Val = 0
    bfs([]*TreeNode{root})
    return max
}
