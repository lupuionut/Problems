package leetcode

/*
	24. Swap Nodes in Pairs
    -----------------------
*/

func swapPairs(head *ListNode) *ListNode {

    nodes := []*ListNode{}
    for n := head; n != nil; n = n.Next {
        nodes = append(nodes, n)
    }

    for i := 0; i < len(nodes) - 1; {
        var temp *ListNode
        temp = nodes[i]
        nodes[i] = nodes[i+1]
        nodes[i+1] = temp
        i += 2
    }

    for i:= 0; i < len(nodes); i++ {
        if i == len(nodes) - 1 {
            (*nodes[i]).Next = nil
        } else {
            (*nodes[i]).Next = nodes[i+1]
        }
    }

    if len(nodes) == 0 {
        return nil
    }
    return nodes[0]
}
