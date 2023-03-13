package leetcode

func IsSymmetric(root *TreeNode) bool {

    left := []*TreeNode{}
    right := []*TreeNode{}

    if root.Left != nil {
        left = append(left, root.Left)
    }

    if root.Right != nil {
        right = append(right, root.Right)
    }

    for true {
        ll := len(left)
        lr := len(right)

        if ll == 0 || lr == 0 {
            break
        }

        if ll != lr {
            return false
        }

        for i := 0; i < ll; i++ {
            if left[i] == nil && right[i] != nil {
                return false
            }
            if left[i] != nil && right[i] == nil {
                return false
            }
            if left[i] != nil && right[i] != nil {
                if left[i].Val != right[i].Val {
                    return false
                }
                left = append(left, left[i].Left)
                left = append(left, left[i].Right)
                right = append(right, right[i].Right)
                right = append(right, right[i].Left)
            }
        }

        left = left[ll:]
        right = right[lr:]
    }

    if len(left) != 0 || len(right) != 0 {
        return false
    }

    return true
}
