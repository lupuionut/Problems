package leetcode

import (
    "sort"
)

func MinDiffInBST(root *TreeNode) int {
    min := 0
    var values IntSliceInterface
    traverseMinDifffInBST(root, &values)
    sort.Sort(values)
    for i := 0; i < len(values) - 1; i++ {
        delta := values[i+1] - values[i]
        if min == 0 || min > delta {
            min = delta
        }
    }
    return min
}

func traverseMinDifffInBST(node *TreeNode, values *IntSliceInterface) {
    if node != nil {
        *values = append(*values, node.Val)
        traverseMinDifffInBST(node.Left, values)
        traverseMinDifffInBST(node.Right, values)
    }
}

type IntSliceInterface []int

func (v IntSliceInterface) Len() int {
    return len(v)
}

func (v IntSliceInterface) Less(i, j int) bool {
    return v[i] < v[j]
}

func (v IntSliceInterface) Swap(i, j int) {
    temp := v[i]
    v[i] = v[j]
    v[j] = temp
}
