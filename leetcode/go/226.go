package leetcode

import "fmt"

func InvertTree(root *TreeNode) *TreeNode {
    if root == nil {
        return nil
    }
    traverseInvertTree(root)
    return root
}

func traverseInvertTree(t *TreeNode) {
    if t == nil {
        return
    }
    temp := t.Left
    t.Left = t.Right
    t.Right = temp
    traverseInvertTree(t.Left)
    traverseInvertTree(t.Right)
}

func (t *TreeNode) String() string {
    str := "["

    if t != nil {
        queue := []*TreeNode{t}
        for true {
            if len(queue) == 0 {
                break
            }
            newqueue := []*TreeNode{}
            for _, n := range(queue) {
                str += fmt.Sprint(n.Val) + ","
                if n.Left != nil {
                    newqueue = append(newqueue, n.Left)
                }
                if n.Right != nil {
                    newqueue = append(newqueue, n.Right)
                }
            }
            queue = newqueue
        }
    }

    str += "]"
    return str
}
