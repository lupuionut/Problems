package leetcode

/*
    1579. Remove Max Number of Edges to Keep Graph Fully Traversable
    ----------------------------------------------------------------
    Use union find to see how many edges we need to have for each person
    to allow traversing the whole graph.
    ua will keep track of connected nodes for alice
    ub will keep track of connected nodes for bob
*/

func maxNumEdgesToRemove(n int, edges [][]int) int {

    connected := 0
    ua := make([]int, n)
    ub := make([]int, n)
    for i := 0; i < n; i++ {
        ua[i] = i
        ub[i] = i
    }

    var find func(x int, parent []int) int
    find = func(x int, parent []int) int {
        if x != parent[x] {
            parent[x] = find(parent[x], parent)
        }
        return parent[x]
    }

    union := func(a, b int, parent []int) {
        pa := find(a, parent)
        pb := find(b, parent)

        if pa != pb {
            parent[pa] = pb
        }
    }

    // start with type 3, to connect nodes
    for k := range edges {
        t, u, v := edges[k][0], edges[k][1], edges[k][2]
        u -= 1
        v -= 1
        if t == 3 {
            if find(u, ua) != find(v, ua) {
                union(u, v, ua)
                union(u, v, ub)
                connected += 1
            }
        }
    }

    // do it also for type 1 and 2
    for k := range edges {
        t, u, v := edges[k][0], edges[k][1], edges[k][2]
        u -= 1
        v -= 1
        if t == 1 {
            if find(u, ua) != find(v, ua) {
                union(u, v, ua)
                connected += 1
            }
        }
        if t == 2 {
            if find(u, ub) != find(v, ub) {
                union(u, v, ub)
                connected += 1
            }
        }
    }

    // check if all nodes have the same parent as the first one
    // if not, it means that all graph nodes cannot be visited
    for i := 0; i < n; i++ {
        if find(i, ua) != find(0, ua) {
            return -1
        }
        if find(i, ub) != find(0, ub) {
            return -1
        }
    }
    return len(edges) - connected
}
