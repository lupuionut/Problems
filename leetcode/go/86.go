package leetcode

/*
    86. Partition List
    ------------------
*/

func partition(head *ListNode, x int) *ListNode {
    left, right := &ListNode{}, &ListNode{}
    ltail, rtail := left, right

    for node := head; node != nil; node = node.Next {
        if node.Val < x {
            ltail.Next = node
            ltail = ltail.Next
        } else {
            rtail.Next = node
            rtail = rtail.Next
        }
    }
    ltail.Next = right.Next
    rtail.Next = nil

    return left.Next
}
