package leetcode

/*
    25. Reverse Nodes in k-Group
    ----------------------------
    Create a k sized slice to keep track of nodes in the current range.
    Reverse the nodes from the slice and make the tail point to recursion.
*/

func reverseKGroup(head *ListNode, k int) *ListNode {
    q := []*ListNode{}

    if head == nil {
        return nil
    }

    // create the slice that contains the nodes in the k range
    i := 0
    for n := head; n != nil; n = n.Next {
        if i == k {
            break
        }
        q = append(q, n)
        i++
    }

    // if the slice is smaller than k, we don't need to reverse
    if len(q) < k {
        return head
    }

    // make head the last node
    head = q[len(q) - 1]
    nextHead := head.Next
    q[0].Next = reverseKGroup(nextHead, k)

    // reverse the nodes
    for i := len(q) - 1; i > 0; i-- {
        q[i].Next = q[i-1]
    }

    return head
}
