package leetcode

func MergeKLists(lists []*ListNode) *ListNode {
    if len(lists) == 0 {
        return nil
    }
    if len(lists) == 1 {
        return lists[0]
    }

    var merged *ListNode
    var previous *ListNode
    left := lists[0]
    right := lists[1]

    if left == nil && right == nil {
        return MergeKLists(lists[2:])
    }

    if left == nil {
        lists = append(lists, right)
        return MergeKLists(lists[2:])
    }

    if right == nil {
        lists = append(lists, left)
        return MergeKLists(lists[2:])
    }

    if left.Val < right.Val {
        merged = &ListNode{Val: left.Val}
        left = left.Next
    } else {
        merged = &ListNode{Val: right.Val}
        right = right.Next
    }
    previous = merged

    for true {
        if left == nil && right == nil {
            break
        }
        if left == nil {
            node := &ListNode{Val: right.Val, Next: right.Next}
            previous.Next = node
            break
        }
        if right == nil {
            node := &ListNode{Val: left.Val, Next: left.Next}
            previous.Next = node
            break
        }
        if left.Val < right.Val {
            node := &ListNode{Val: left.Val}
            previous.Next = node
            previous = previous.Next
            left = left.Next
        } else {
            node := &ListNode{Val: right.Val}
            previous.Next = node
            previous = previous.Next
            right = right.Next
        }
    }
    lists = append(lists, merged)

    return MergeKLists(lists[2:])
}
