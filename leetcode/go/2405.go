package leetcode

/*
    2405. Optimal Partition of String
    ---------------------------------
    Iterate the string and keep track of visited letters into a map.
    Whenever the map counts that letter more than 0, it means we must start
    a new substring.
*/

func partitionString(s string) int {
    m := 0
    visited := make(map[byte]int)

    for i := 0; i < len(s); i++ {
        if visited[s[i]] != 0 {
            visited = make(map[byte]int)
        }
        if len(visited) == 0 {
            m += 1
        }
        visited[s[i]] = 1
    }

    return m
}
