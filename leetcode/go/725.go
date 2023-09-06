// 725. Split Linked List in Parts
// -------------------------------
package leetcode

func splitListToParts(head *ListNode, k int) []*ListNode {
    var ans []*ListNode

    l := 0
    t := head

    for {
        if t == nil {
            break
        }
        t = t.Next
        l += 1
    }

    minNodePerBucket := l / k
    extraNodes := l % k

    current := head
    var previous *ListNode

    for k > 0 {
        if current == nil {
            ans = append(ans, nil)
        } else {
            for i := 0; i < minNodePerBucket; i++ {
                previous = current
                current = current.Next
            }
            if extraNodes > 0 {
                previous = current
                current = current.Next
                extraNodes -= 1
            }
            previous.Next = nil
            ans = append(ans, head)
            head = current
        }
        k -= 1
    }

    return ans
}
