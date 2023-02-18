package leetcode

func ShortestAlternatingPaths(n int, redEdges [][]int, blueEdges [][]int) []int {

    redMap := make(map[int][]int)
    blueMap := make(map[int][]int)
    coloursMap := [2]map[int][]int{redMap, blueMap}
    result := []int{0}
    queue := []int{0}

    for _, v := range(redEdges) {
        insertIntoMap(coloursMap[0], v[0], v[1])
    }

    for _, v := range(blueEdges) {
        insertIntoMap(coloursMap[1], v[0], v[1])
    }

    for k := 1; k < n; k++ {
        visited := [2][]int{}
        r0 := ShortestAlternatingPathsBFS(queue, k, 0, 0, visited, coloursMap)
        r1 := ShortestAlternatingPathsBFS(queue, k, 1, 0, visited, coloursMap)
        if r0 == -1 && r1 == -1 {
            result = append(result, -1)
            continue
        }
        if r0 != -1 && r1 != -1 {
            if r0 < r1 {
                result = append(result, r0)
            } else {
                result = append(result, r1)
            }
        }
        if r0 == -1 {
            result = append(result, r1)
        }
        if r1 == -1 {
            result = append(result, r0)
        }
    }

    return result
}

func ShortestAlternatingPathsBFS(queue []int, t, col, lvl int, visited [2][]int, coloursMap [2]map[int][]int) int {
    if len(queue) == 0 {
        return -1
    }
    newqueue := []int{}
    for _, n := range(queue) {
        if sliceContains(visited[col], n) {
            continue 
        }
        visited[col] = append(visited[col], n)
        if nodes, ok := coloursMap[col][n]; ok {
            if sliceContains(nodes, t) {
                return lvl + 1
            } else {
                for _, node := range(nodes) {
                    newqueue = append(newqueue, node)
                }
            }
        }
    }
    if len(newqueue) > 0 {
        return ShortestAlternatingPathsBFS(newqueue, t, col ^ 1, lvl + 1, visited, coloursMap)
    }

    return -1
}

func insertIntoMap(m map[int][]int, key, val int) {
    if v, ok := m[key]; ok {
        m[key] = append(v, val)
    } else {
        m[key] = []int{val}
    }
}

func sliceContains(xs []int, x int) bool {
    for _, v := range(xs) {
        if x == v {
            return true
        }
    }
    return false
}
