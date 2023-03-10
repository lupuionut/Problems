package leetcode

import (
    //"fmt"
    "time"
)

type Solution struct {
    Nodes *ListNode
    Cursor *ListNode
}

func SolutionConstructor(head *ListNode) Solution {
    return Solution{Nodes: head, Cursor: head}
}

func (this *Solution) GetRandom() int {
    counter := time.Now().UnixMicro() & 0xfff

    for counter > 0 {
        if this.Cursor.Next == nil {
            this.Cursor = this.Nodes
        } else {
            this.Cursor = this.Cursor.Next
        }
        counter -= 1
    }

    return this.Cursor.Val
}
