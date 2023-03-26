package leetcode
/*
    this uses Floyd's algo to find if thre is a cycle and calculates the cycle
    https://en.wikipedia.org/wiki/Cycle_detection
*/

func LongestCycle(edges []int) int {

    // keep previous results to be used for each iteration
    results := make(map[int]int)
    max := -1

    for k := range edges {
        length := detectCycle(k, edges, results)
        results[k] = length
        if length > max {
            max = length
        }
    }

    return max
}

func detectCycle(start int, edges []int, results map[int]int) int {
    next := edges[start]
    steps := 1

    if next != - 1 {
        nnext := edges[next]
        for {
            // if next or next.next == -1, it means that there is no cycle
            if next == -1 || nnext == -1 {
                return -1
            }

            // if we already visited any of the next, or next.next nodes
            // and we find a cycle, return that cycle value
            if results[next] != 0 {
                return results[next]
            }
            if results[nnext] != 0 {
                return results[nnext]
            }

            // if next == next.next there is a cycle
            if nnext == next {
                s := start
                position := 0
                for {
                    // if we don't cycle multiple times
                    // the length of the cycle = total steps - cycle start position
                    if s == next {
                        return steps - position
                    } else {
                        // if we cycle multiple times until we meet s with next
                        // the length of the cycle will equal position
                        if position > 0 && next == nnext {
                            return position
                        }
                        next = edges[next]
                        s = edges[s]
                        position += 1
                        steps += 1
                    }
                }
            } else {
                next = edges[next]
                nk := edges[nnext]
                steps += 1
                if nk == -1 {
                    return  -1
                } else {
                    nnext = edges[nk]
                }
            }
        }
    }
    return -1
}
