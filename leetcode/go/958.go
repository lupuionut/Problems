package leetcode

func IsCompleteTree(root *TreeNode) bool {
    var line []int
    q := []*TreeNode{root}
    i := 0

    for true {
        line = []int{}
        length := len(q)
        last := true

        for k := range q {
           if q[k] == nil {
                line = append(line, 0)
           } else {
                if q[k].Left != nil || q[k].Right != nil {
                    last = false
                }
                line = append(line, q[k].Val)
                q = append(q, q[k].Left)
                q = append(q, q[k].Right)
           }
        }

        q = q[length:]
        if last {
            break
        }
        i += 1
    }

    if i == 0 {
        return true
    }

    if len(line) != 1 << i {
        return false
    }

    ended := false
    for j := 0 ; j < len(line); j++ {
        if ended && line[j] != 0 {
            return false
        }
        if line[j] == 0 {
            ended = true
        }
    }

    return true
}
