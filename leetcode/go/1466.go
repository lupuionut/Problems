package leetcode

func MinReorder(n int, connections [][]int) int {
    relations := make(map[int][]int)
    directions := make(map[int][]int)
    changes := 0

    for k := range connections {
        node := connections[k]
        relations[node[0]] = append(relations[node[0]], node[1])
        relations[node[1]] = append(relations[node[1]], node[0])
        directions[node[0]] = append(directions[node[0]], node[1])
    }

    q := []int{0}
    visited := make([]int, n)

    for {
        length := len(q)
        if length == 0 {
            break
        }
        for k := range q {
            visited[q[k]] = 1

            connected := relations[q[k]]

            for i := range connected {
                node := connected[i]

                ok := false
                for j := range directions[node] {
                    if directions[node][j] == q[k] {
                        ok = true
                        break
                    }
                }

                if ok == false && visited[node] == 0 {
                    changes += 1
                }
                if visited[node] == 0 {
                    q = append(q, node)
                }
            }
        }
        q = q[length:]
    }

    return changes
}
