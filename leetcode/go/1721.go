package leetcode

/*
    1721. Swapping Nodes in a Linked List
    -------------------------------------
*/

func swapNodes(head *ListNode, k int) *ListNode {
    l := 1
    var first *ListNode
    var second *ListNode

    n := head
    for {
        if n == nil {
            l -= 1
            break
        }
        if l == k {
            first = n
        }
        l += 1
        n = n.Next
    }

    n = head
    for {
        if n == nil {
            break
        }
        if l - k == 0 {
            second = n
            break
        }
        l -= 1
        n = n.Next
    }

    first.Val, second.Val = second.Val, first.Val
    return head
}
