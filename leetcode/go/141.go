/*
    141. Linked List Cycle
    ----------------------
*/

func hasCycle(head *ListNode) bool {

    if head == nil || head.Next == nil {
        return false
    }

    f := head
    s := head.Next

    for {
        if s == nil || s.Next == nil {
            break
        }
        if f == s {
            return true
        }
        f = f.Next
        s = s.Next.Next
    }

    return false
}
