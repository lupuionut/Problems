package leetcode

/*
    133. Clone Graph
    ----------------
*/
type Node1 struct {
    Val int
    Neighbors []*Node1
}

func cloneGraph(node *Node1) *Node1 {
    if node == nil {
        return nil
    }

    cloned := make(map[int]*Node1)
    cloned[node.Val] = &Node1{Val: node.Val, Neighbors: []*Node1{}}

    queue := []*Node1{}
    queue = append(queue, node)

    for len(queue) > 0 {
        last := len(queue) - 1
        cur := queue[last]
        queue = queue[0:last]

        for k := range cur.Neighbors {
            neighbor := cur.Neighbors[k]
            if _, ok := cloned[neighbor.Val]; !ok {
                cloned[neighbor.Val] = &Node1{Val: neighbor.Val, Neighbors: []*Node1{}}
                queue = append(queue, neighbor)
            }
            cloned[cur.Val].Neighbors = append(cloned[cur.Val].Neighbors, cloned[neighbor.Val]) 
        }
    }

    return cloned[node.Val]
}
