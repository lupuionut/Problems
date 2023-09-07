// 92. Reverse Linked List II
// --------------------------

package leetcode

func reverseBetween(head *ListNode, left int, right int) *ListNode {
    dummy := &ListNode{Val: 0, Next: head}
    var prev *ListNode
    curr := dummy

    for i := 0; i < left; i++ {
        prev = curr
        curr = curr.Next
    }

    leftNode := prev
    leftNode.Next = nil
    head = curr

    for i := left; i <= right; i++ {
        prev = curr
        curr = curr.Next
    }

    prev.Next = nil
    prev = curr

    for head != nil {
        next := head.Next
        head.Next = prev
        prev = head
        head = next
    }
    leftNode.Next = prev

    return dummy.Next
}
