package leetcode

/*
    create a hashmap with node -> nodes
    traverse all nodes and check for min
*/
func minScore(n int, roads [][]int) int {

    min := 0
    hm := make(map[int][][2]int)

    for i := 0; i < len(roads); i++ {
        k0 := roads[i][0]
        k1 := roads[i][1]
        node0 := [2]int{roads[i][1], roads[i][2]}
        node1 := [2]int{roads[i][0], roads[i][2]}
        if nodes, ok := hm[k0]; ok {
            hm[k0] = append(nodes, node0)
        } else {
            hm[k0] = [][2]int{node0}
        }
        if nodes, ok := hm[k1]; ok {
            hm[k1] = append(nodes, node1)
        } else {
            hm[k1] = [][2]int{node1}
        }
    }

    q := []int{1, n}
    visited := make(map[int]interface{})

    for true {
        length := len(q)
        if length == 0 {
            break
        }

        for k := range q {
            if _, ok := visited[q[k]]; !ok {
                visited[q[k]] = true
                childrens := hm[q[k]]
                for c := range childrens {
                    if min == 0 || min > childrens[c][1] {
                        min = childrens[c][1]
                    }
                    q = append(q, childrens[c][0])
                }
            }
        }

        q = q[length:]
    }

    return min
}
