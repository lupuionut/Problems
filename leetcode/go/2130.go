package leetcode

/*
    2130. Maximum Twin Sum of a Linked List
    ---------------------------------------
    Find the length off the list, and keep track
    of the first half values so we can sum them
    with the ones from the second half.
*/

func pairSum(head *ListNode) int {
    length := 0
    for n := head; n != nil; n = n.Next {
        length += 1
    }
    results := make([]int, length/2)
    i, max := 0, 0
    for n := head; n != nil; n = n.Next {
        if i >= length/2 {
            j := length - i - 1
            t := results[j] + n.Val
            if t > max {
                max = t
            }
        } else {
            results[i] = n.Val
        }
        i += 1
    }
    return max
}
