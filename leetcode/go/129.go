package leetcode

import (
    "math"
)

func SumNumbers(root *TreeNode) int {
    acc := []int{}
    sum := new(int)
    traverseWithAcc(root, acc, sum)
    return *sum
}

func traverseWithAcc (n *TreeNode, acc []int, sum *int) {
    acc = append(acc, n.Val)
    if n.Left == nil && n.Right == nil {
        v := traverseWithAccToInt(acc)
        *sum += v
    }
    if n.Left != nil {
        traverseWithAcc(n.Left, acc, sum)
    }
    if n.Right != nil {
        traverseWithAcc(n.Right, acc, sum)
    }
}

func traverseWithAccToInt(xs []int) int {
    l := len(xs)
    t := 0
    for k := range xs {
        t += xs[k] * int(math.Pow10(l-k-1))
    }
    return t
}
