package leetcode

import "fmt"

type ListNode struct {
    Val int
    Next *ListNode
}

func DetectCycle(head *ListNode) *ListNode {
    if head == nil {
        return nil
    }

    visited := make(map[string]bool)

    for head.Next != nil {
        location := fmt.Sprint(&head.Next)
        if _, ok := visited[location]; ok {
            return head
        } else {
            visited[location] = true
        }
        head = head.Next
    }
    return nil
}
