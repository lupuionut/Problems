package leetcode

/*
    61. Rotate List
    ---------------
    Use a slice to have an easy access to the index of each node
    from the list.
    When the rotate is equal with 0 or the length or multiple of
    list length, don't rotate, and just return the head.
    Otherwise, make the last node from list point to the head and
    the node before the new head point to nil.
*/

func rotateRight(head *ListNode, k int) *ListNode {

    if head == nil {
        return nil
    }

    positions := []*ListNode{}
    for node := head; node != nil; node = node.Next {
        positions = append(positions, node)
    }

    p := len(positions)

    if k > p {
        k = k%p
    }
    if (p - k) == 0 || k == 0 {
        return head
    }
    newHead := p - k
    positions[p-1].Next = head
    positions[newHead - 1].Next = nil

    return positions[newHead]
}
