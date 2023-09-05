// 138. Copy List with Random Pointer
// ----------------------------------

func copyRandomList(head *Node) *Node {
    dummy := &Node{Val: -1, Next: head}
    current := dummy
    newHead := &Node{Val: -1}
    newCurrent := newHead
    cache := make(map[*Node]*Node)

    var getCopy func(n *Node) *Node
    getCopy = func(n *Node) *Node {
        if n == nil {
            return nil
        }
        if val, ok := cache[n]; ok {
            return val
        }
        copy := &Node{Val: n.Val}
        cache[n] = copy
        return copy
    }

    for {
        if current == nil {
            break
        }
        newCurrent.Next = getCopy(current.Next)
        newCurrent.Random = getCopy(current.Random)
        current = current.Next
        newCurrent = newCurrent.Next
    }

    return newHead.Next
}
