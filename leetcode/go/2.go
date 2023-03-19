package leetcode

func AddTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
    return AddTwoNumbersRec(l1, l2, 0)
}

func AddTwoNumbersRec(l1 *ListNode, l2 *ListNode, carry int) *ListNode {
    if l1 == nil && l2 == nil {
        if carry == 1 {
            return &ListNode{Val: 1}
        }
        return nil
    }

    a, b, sum := 0,0,0

    if l1 != nil {
        a = l1.Val
        l1 = l1.Next
    }
    if l2 != nil {
        b = l2.Val
        l2 = l2.Next
    }

    sum = a + b + carry
    if sum >= 10 {
        carry = 1
    } else {
        carry = 0
    }
    return &ListNode{Val: sum%10, Next: AddTwoNumbersRec(l1, l2, carry)}
}
