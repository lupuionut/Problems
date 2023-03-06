package leetcode

/*
    Do a BFS by composing a queue with the idx-1, idx+1 and the idx
    for the first occurence of the same value.
    For each pass, add + 1 to the counter.
    When any of the idx will equal the len of arr, we finished.
*/
func MinJumps(arr []int) int {
    type Node struct {
        Val int
        Idx int
    }

    if len(arr) == 1 {
        return 0
    }
    visited := make([]bool, len(arr))
    siblings := make(map[int][]int)

    for i := 0; i < len(arr); i++ {
        if val, ok := siblings[arr[i]]; ok {
            siblings[arr[i]] = append(val, i)
        } else {
            siblings[arr[i]] = []int{i}
        }
        visited[i] = false
    }

    root := Node{Val: arr[0], Idx: 0}
    queue := []Node{root}
    counter := 0

    loop:
    for true {
        // if we don't have a queue we finished
        lq := len(queue)
        if lq == 0 {
            break
        }

        // go through each possible idx value
        for k := range queue {
            if visited[queue[k].Idx] == true {
                continue
            }
            // if the queue idx is equal with the value of the last idx in arr
            // we finished
            if queue[k].Idx == len(arr) - 1 {
                break loop
            } else {
                // add - 1
                newq := queue[k].Idx - 1
                if newq > 0 && visited[newq] == false {
                    queue = append(queue, Node{Val: arr[queue[k].Idx - 1], Idx: queue[k].Idx - 1})
                }

                newq = queue[k].Idx + 1
                // add + 1
                if newq < len(arr) && visited[newq] == false {
                    queue = append(queue, Node{Val: arr[queue[k].Idx + 1], Idx: queue[k].Idx + 1})
                }

                // add first possible sibling and reduce the map with 1
                if v, ok := siblings[queue[k].Val]; ok {
                    for i := range v {
                        if v[i] != queue[k].Idx {
                            queue = append(queue, Node{Val: queue[k].Val, Idx: v[i]})
                            siblings[queue[k].Val] = v[i+1:]
                        }
                    }

                }
            }
            visited[queue[k].Idx] = true
        }
        queue = queue[lq:]
        counter += 1
    }

    return counter
}
