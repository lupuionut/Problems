package leetcode

/*
    743. Network Delay Time
    -----------------------
    Dijkstra's algorithm
*/

import "container/heap"

type Pair struct {
    V int
    W int
    I int
}
type PairHeap []*Pair
func (h PairHeap) Len() int {
    return len(h)
}
func (h PairHeap) Less(i, j int) bool {
    return h[i].W < h[j].W
}
func (h PairHeap) Swap(i, j int) {
    h[i], h[j] = h[j], h[i]
    h[i].I = i
    h[j].I = j
}
func (h *PairHeap) Push(x any) {
	n := len(*h)
	p := x.(*Pair)
	p.I = n
	*h = append(*h, p)
}
func (h *PairHeap) Pop() any {
	old := *h
	n := len(old)
	p := old[n-1]
	old[n-1] = nil
	p.I = -1
	*h = old[0 : n-1]
	return p
}

func networkDelayTime(times [][]int, n int, k int) int {

    adj := make(map[int][]*Pair)
    for i := range times {
        u := times[i][0]
        v := times[i][1]
        w := times[i][2]
        if len(adj[u]) == 0 {
            adj[u] = []*Pair{}
        }
        p := &Pair{V: v, W: w}
        adj[u] = append(adj[u], p)
    }

    minheap := &PairHeap{}
    heap.Init(minheap)
    heap.Push(minheap, &Pair{W: 0, V: k, I: 0})
    time := 0
    visited := make(map[int]int)

    for minheap.Len() > 0 {
        node := heap.Pop(minheap).(*Pair)
        if visited[node.V] == 1 {
            continue
        }
        visited[node.V] = 1
        if node.W > time {
            time = (*node).W
        }

        for k := range adj[node.V] {
            neighbour := adj[node.V][k]
            if visited[neighbour.V] == 1 {
                continue
            }
            neighbour.W += node.W
            heap.Push(minheap, neighbour)
        }
    }

    if len(visited) != n {
        return -1
    }
    return time
}
