package leetcode

func makeConnected(n int, connections [][]int) int {
    if len(connections) < n - 1 {
        return -1
    }

    visited := make([]int, n)
    adj := make(map[int][]int)

    for k := range connections {
        k0 := connections[k][0]
        k1 := connections[k][1]
        adj[k0] = append(adj[k0], k1)
        adj[k1] = append(adj[k1], k0)
    }

    connected := 0

    for i := 0; i < n; i++ {
        if visited[i] != 1 {
            connected += 1
            makeConnecteddfs(i, visited, adj)
        }
    }

    return connected - 1
}

func makeConnecteddfs(i int, visited []int, adj map[int][]int) {
    visited[i] = 1
    for k := range adj[i] {
        neighbour := adj[i][k]
        if visited[neighbour] == 0 {
            makeConnecteddfs(neighbour, visited, adj)
        }
    }
}
