package leetcode

func MinimumFuelCost(roads [][]int, seats int) int {
    liters := 0
    routes := make(map[int][]int)    
    
    for _, road := range(roads) {
        a := road[0]
        b := road[1]
        if r, ok := routes[a]; ok {
            routes[a] = append(r, b)
        } else {
            routes[a] = []int{b}
        }
        if rb, ok := routes[b]; ok {
            routes[b] = append(rb, a)
        } else {
            routes[b] = []int{a}
        }
    }

    roots := routes[0]
    personsAtLevel := make(map[int]int)
    
    for _, node := range(roots) {
        liters += traverse(node, 0, routes, personsAtLevel, seats)
    }

    return liters
}

func traverse(node int, parent int, routes map[int][]int, personsAtLevel map[int]int, seats int) int {
    nodes := routes[node]
    liters := 1
    childs := 0
    
    if len(nodes) >= 1 {
        for _, n := range(nodes) {
            if n != parent {
                childs += traverse(n, node, routes, personsAtLevel, seats)
            }
        }
    } 
    personsAtLevel[node] += 1
    personsAtLevel[parent] += personsAtLevel[node]
    liters = personsAtLevel[node]/seats
    if personsAtLevel[node]%seats != 0 {
        liters += 1
    }

    return childs + liters
}
