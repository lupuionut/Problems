package leetcode

/*
    collect all the values from the linked list
    and do a binary search to create multiple treenodes
*/
func SortedListToBST(head *ListNode) *TreeNode {
    if head == nil {
        return nil
    }

    values := []int{}
    for n := head; n != nil; n = n.Next {
        values = append(values, n.Val)
    }

    mid := len(values)/2
    root := &TreeNode{Val: values[mid], Left: buildFromValues(values[0:mid]), Right: buildFromValues(values[mid+1:])}

    return root
}

func buildFromValues(xs []int) *TreeNode {
    if len(xs) == 0 {
        return nil
    }
    if len(xs) == 1 {
        return &TreeNode{Val: xs[0]}
    }
    if len(xs) == 2 {
        return &TreeNode{Val: xs[1], Left: &TreeNode{Val: xs[0]}}
    }
    m := len(xs)/2
    return &TreeNode{Val: xs[m], Left: buildFromValues(xs[0:m]), Right: buildFromValues(xs[m+1:])}
}
