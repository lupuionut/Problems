package leetcode

import (
    "math/rand"
    "time"
)

type Solution struct {
    Nodes []int
}

func Constructor(head *ListNode) Solution {
    s := Solution{}
    for a := head; a != nil; a = a.Next {
        s.Nodes = append(s.Nodes, a.Val)
    }
    return s
}

func (this *Solution) GetRandom() int {
    rand.Seed(time.Now().UnixNano())
    return this.Nodes[rand.Intn(len(this.Nodes))]
}

