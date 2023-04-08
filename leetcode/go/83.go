package leetcode

/*
    83. Remove Duplicates from Sorted List
    --------------------------------------
*/

func deleteDuplicates(head *ListNode) *ListNode {

    if head == nil {
        return nil
    }
    if head.Next == nil {
        return head
    }
    if head.Next.Val == head.Val {
        return deleteDuplicates(head.Next)
    }
    return &ListNode{Val: head.Val, Next: deleteDuplicates(head.Next)}
}
