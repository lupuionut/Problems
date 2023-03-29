package leetcode

/*
    19. Remove Nth Node From End of List
    ------------------------------------
    Make a slice with each list node, find the index of the node to be removed
    and make the node before to have next pointing to the node after the one
    removed.
*/
func removeNthFromEnd(head *ListNode, n int) *ListNode {
    nodes := []*ListNode{}

    for n := head; n != nil; n = n.Next {
        nodes = append(nodes, n)
    }

    k := len(nodes) - n
    if k == 0 {
        return nodes[0].Next
    }
    nodes[k-1].Next = nodes[k].Next
    return head
}
