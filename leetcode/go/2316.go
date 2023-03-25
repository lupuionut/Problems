package leetcode

/*
    We must find the number of sets not connected.
    A set is one or more nodes connected, so we will keep track only of number
    of nodes connected into a set not of actual nodes (sets map[int]*int).

    To start, for each edge, add connected nodes into a map (hm).

    Do a dfs over each node from hm and recurse over connected nodes.
    When finished, we have a set and we can increase the number of sets with 1.

    Iterate over the sets, and calculate the number of unreacheable pairs.
    for ex: 3 sets, with 4 nodes, 2 nodes, 1 node => 4x2 + 4x1 + 2x1 = 14
*/
func CountPairs(n int, edges [][]int) int {
    hm := make(map[int][]int)
    visited := make([]int, n)
    sets := make(map[int]*int)

    for k := range edges {
        edge := edges[k]
        hm[edge[0]] = append(hm[edge[0]], edge[1])
        hm[edge[1]] = append(hm[edge[1]], edge[0])
    }

    j := 0
    for i := 0; i < n; i++ {
        if _, ok := sets[j]; !ok {
            sets[j] = new(int)
        }
        if visited[i] == 0 {
            countPairsDFS(i, visited, hm, sets[j])
            j += 1
        }
    }

    // for each iteration we add to acc the number of nodes contained by that set
    // this way we calculate the unreachable nodes as number of nodes present in current set
    // *sets[i] multiplied by the remaining number of nodes (n - acc)
    count := 0
    acc := 0
    for i := 0; i < len(sets); i++ {
        acc += (*sets[i])
        count += (*sets[i]) * (n - acc)
    }

    return count
}

func countPairsDFS(i int, visited []int, hm map[int][]int, setSize *int) {
    *setSize += 1
    visited[i] = 1
    childs := hm[i]
    for k := range childs {
        if visited[childs[k]] == 0 {
            countPairsDFS(childs[k], visited, hm, setSize)
        }
    }
}
